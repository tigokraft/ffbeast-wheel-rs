use hidapi::HidApi;

use crate::constants::{PID, REPORT_SIZE, VID};
use crate::enums::{field_type_for, FieldType, ReportData, ReportType, SettingField};
use crate::error::WheelError;
use crate::types::{
    AdcSettings, DeviceSettings, DeviceState, DirectControl, EffectSettings, FirmwareLicense,
    FirmwareVersion, GpioSettings, HardwareSettings,
};
use crate::utils::{convert_position_to_degrees, normalize_torque, StructReader};

// -----------------------------------------------------------------------
// Pure buffer-building helpers (pub(crate) so tests can reach them)
// -----------------------------------------------------------------------

/// Builds the 65-byte HID output buffer for a setting-field write.
///
/// Buffer layout:
/// ```text
/// [0]    report ID        = 0xA3 (GenericInputOutput)
/// [1]    report data      = 0x14 (DataSettingsFieldData)
/// [2]    field identifier
/// [3]    field index
/// [4…]   value encoded as the field's native type (little-endian)
/// ```
pub(crate) fn build_setting_buf(field: SettingField, index: u8, value: i64) -> [u8; REPORT_SIZE + 1] {
    let mut buf = [0u8; REPORT_SIZE + 1];
    buf[0] = ReportType::GenericInputOutput as u8;
    buf[1] = ReportData::DataSettingsFieldData as u8;
    buf[2] = field as u8;
    buf[3] = index;

    const VAL: usize = 4;
    match field_type_for(field) {
        FieldType::Int8 => {
            buf[VAL] = value.clamp(-128, 127) as i8 as u8;
        }
        FieldType::Uint8 => {
            buf[VAL] = value.clamp(0, 255) as u8;
        }
        FieldType::Int16 => {
            buf[VAL..VAL + 2]
                .copy_from_slice(&(value.clamp(-32768, 32767) as i16).to_le_bytes());
        }
        FieldType::Uint16 => {
            buf[VAL..VAL + 2]
                .copy_from_slice(&(value.clamp(0, 65535) as u16).to_le_bytes());
        }
        FieldType::Float32 => {
            buf[VAL..VAL + 4].copy_from_slice(&(value as f32).to_le_bytes());
        }
        FieldType::Int32 => {
            buf[VAL..VAL + 4].copy_from_slice(
                &(value.clamp(i32::MIN as i64, i32::MAX as i64) as i32).to_le_bytes(),
            );
        }
        FieldType::Uint32 => {
            buf[VAL..VAL + 4]
                .copy_from_slice(&(value.clamp(0, u32::MAX as i64) as u32).to_le_bytes());
        }
    }
    buf
}

/// Builds the 65-byte HID output buffer for a direct-control write.
///
/// Buffer layout:
/// ```text
/// [0]    report ID     = 0xA3
/// [1]    report data   = 0x10 (DataOverrideData)
/// [2–3]  spring_force  (i16 LE, clamped to ±10 000)
/// [4–5]  constant_force (i16 LE, clamped to ±10 000)
/// [6–7]  periodic_force (i16 LE, clamped to ±10 000)
/// [8]    force_drop    (u8, clamped to 0–100)
/// ```
pub(crate) fn build_direct_control_buf(control: &DirectControl) -> [u8; REPORT_SIZE + 1] {
    let mut buf = [0u8; REPORT_SIZE + 1];
    buf[0] = ReportType::GenericInputOutput as u8;
    buf[1] = ReportData::DataOverrideData as u8;
    buf[2..4].copy_from_slice(&control.spring_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[4..6].copy_from_slice(&control.constant_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[6..8].copy_from_slice(&control.periodic_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[8] = control.force_drop.min(100);
    buf
}

/// Parses a license key string into three `u32` chunks.
///
/// Expected format: `"XXXXXXXX-XXXXXXXX-XXXXXXXX"` (three 8-character
/// uppercase or lowercase hex segments separated by hyphens).
///
/// Returns `Err(WheelError::InvalidLicense)` on any format violation.
pub(crate) fn parse_license_key(license: &str) -> Result<[u32; 3], WheelError> {
    let chunks: Vec<&str> = license.trim().split('-').collect();
    if chunks.len() != 3 {
        return Err(WheelError::InvalidLicense(
            "expected 3 hyphen-separated segments".into(),
        ));
    }
    let mut out = [0u32; 3];
    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.len() != 8 || !chunk.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(WheelError::InvalidLicense(format!(
                "segment {} must be exactly 8 hex characters (got {:?})",
                i + 1,
                chunk
            )));
        }
        out[i] = u32::from_str_radix(chunk, 16)
            .map_err(|_| WheelError::InvalidLicense(format!("cannot parse segment {}", i + 1)))?;
    }
    Ok(out)
}

/// HID-based API for communicating with the FFBeast wheel controller.
///
/// Wraps a live [`hidapi::HidDevice`] connection and exposes typed methods
/// for reading settings, streaming device state, and sending commands.
///
/// # Example
///
/// ```no_run
/// use ffbeast_wheel_api::WheelApi;
///
/// let mut wheel = WheelApi::connect().expect("connect failed");
///
/// let settings = wheel.read_all_settings().expect("read failed");
/// println!("{:?}", settings.effects);
///
/// wheel.listen(|state| {
///     println!("pos={} torque={}", state.position, state.torque);
///     true // returning false stops the loop
/// }).expect("listen failed");
/// ```
pub struct WheelApi {
    device: hidapi::HidDevice,
    cached_motion_range: Option<u16>,
    last_state: Option<DeviceState>,
}

impl WheelApi {
    // -----------------------------------------------------------------------
    // Construction
    // -----------------------------------------------------------------------

    /// Connects to the first FFBeast wheel found on vendor interface 0.
    ///
    /// Caches the motion range from effect settings immediately after
    /// connecting so that [`DeviceState::position_degrees`] is populated
    /// from the very first state report.
    pub fn connect() -> Result<Self, WheelError> {
        let api = HidApi::new()?;

        let path = api
            .device_list()
            .find(|d| {
                d.vendor_id() == VID && d.product_id() == PID && d.interface_number() == 0
            })
            .ok_or(WheelError::DeviceNotFound)?
            .path()
            .to_owned();

        let device = api.open_path(&path)?;

        let mut wheel = Self {
            device,
            cached_motion_range: None,
            last_state: None,
        };

        // Cache motion range so position-in-degrees is available immediately.
        if let Ok(s) = wheel.read_effect_settings() {
            wheel.cached_motion_range = Some(s.motion_range);
        }

        Ok(wheel)
    }

    // -----------------------------------------------------------------------
    // Computed accessors
    // -----------------------------------------------------------------------

    /// Last device state received via [`read_state`] or [`listen`].
    pub fn last_state(&self) -> Option<&DeviceState> {
        self.last_state.as_ref()
    }

    /// Motion range cached at connect time and updated by [`send_setting`].
    pub fn cached_motion_range(&self) -> Option<u16> {
        self.cached_motion_range
    }

    /// Current wheel position in degrees, or `None` if unavailable.
    pub fn position_degrees(&self) -> Option<f64> {
        let state = self.last_state.as_ref()?;
        let range = self.cached_motion_range?;
        Some(convert_position_to_degrees(state.position, range))
    }

    /// Current torque normalised to [-100.0, 100.0], or `None` if unavailable.
    pub fn torque_normalized(&self) -> Option<f64> {
        Some(normalize_torque(self.last_state.as_ref()?.torque))
    }

    /// Firmware version string (e.g. `"1.2.3"`), or `None` if unavailable.
    pub fn firmware_version(&self) -> Option<String> {
        let v = &self.last_state.as_ref()?.firmware_version;
        Some(format!("{}.{}.{}", v.major, v.minor, v.patch))
    }

    /// Firmware release type, or `None` if unavailable.
    pub fn firmware_release_type(&self) -> Option<u8> {
        Some(self.last_state.as_ref()?.firmware_version.release_type)
    }

    // -----------------------------------------------------------------------
    // Reading settings
    // -----------------------------------------------------------------------

    /// Reads effect settings from the device.
    pub fn read_effect_settings(&self) -> Result<EffectSettings, WheelError> {
        let buf = self.get_feature_report(ReportType::EffectSettingsFeature as u8)?;
        let mut r = StructReader::new(&buf);
        Ok(EffectSettings {
            motion_range: r.u16()?,
            static_dampening_strength: r.u16()?,
            soft_stop_dampening_strength: r.u16()?,
            total_effect_strength: r.u8()?,
            integrated_spring_strength: r.u8()?,
            soft_stop_range: r.u8()?,
            soft_stop_strength: r.u8()?,
            direct_x_constant_direction: r.i8()?,
            direct_x_spring_strength: r.u8()?,
            direct_x_constant_strength: r.u8()?,
            direct_x_periodic_strength: r.u8()?,
        })
    }

    /// Reads hardware settings from the device.
    pub fn read_hardware_settings(&self) -> Result<HardwareSettings, WheelError> {
        let buf = self.get_feature_report(ReportType::HardwareSettingsFeature as u8)?;
        let mut r = StructReader::new(&buf);
        Ok(HardwareSettings {
            encoder_cpr: r.u16()?,
            integral_gain: r.u16()?,
            proportional_gain: r.u8()?,
            force_enabled: r.u8()?,
            debug_torque: r.u8()?,
            amplifier_gain: r.u8()?,
            calibration_magnitude: r.u8()?,
            calibration_speed: r.u8()?,
            power_limit: r.u8()?,
            braking_limit: r.u8()?,
            position_smoothing: r.u8()?,
            speed_buffer_size: r.u8()?,
            encoder_direction: r.i8()?,
            force_direction: r.i8()?,
            pole_pairs: r.u8()?,
        })
    }

    /// Reads GPIO extension settings from the device.
    pub fn read_gpio_extension_settings(&self) -> Result<GpioSettings, WheelError> {
        let buf = self.get_feature_report(ReportType::GpioSettingsFeature as u8)?;
        let mut r = StructReader::new(&buf);

        let extension_mode = r.u8()?;

        let mut pin_mode = [0u8; 10];
        for v in &mut pin_mode {
            *v = r.u8()?;
        }

        let mut button_mode = [0u8; 32];
        for v in &mut button_mode {
            *v = r.u8()?;
        }

        Ok(GpioSettings {
            extension_mode,
            pin_mode,
            button_mode,
            spi_mode: r.u8()?,
            spi_latch_mode: r.u8()?,
            spi_latch_delay: r.u8()?,
            spi_clk_pulse_length: r.u8()?,
        })
    }

    /// Reads ADC extension settings from the device.
    pub fn read_adc_extension_settings(&self) -> Result<AdcSettings, WheelError> {
        let buf = self.get_feature_report(ReportType::AdcSettingsFeature as u8)?;
        let mut r = StructReader::new(&buf);

        let mut r_axis_min = [0u16; 3];
        for v in &mut r_axis_min {
            *v = r.u16()?;
        }
        let mut r_axis_max = [0u16; 3];
        for v in &mut r_axis_max {
            *v = r.u16()?;
        }
        let mut r_axis_smoothing = [0u8; 3];
        for v in &mut r_axis_smoothing {
            *v = r.u8()?;
        }
        let mut r_axis_to_button_low = [0u8; 3];
        for v in &mut r_axis_to_button_low {
            *v = r.u8()?;
        }
        let mut r_axis_to_button_high = [0u8; 3];
        for v in &mut r_axis_to_button_high {
            *v = r.u8()?;
        }
        let mut r_axis_invert = [0u8; 3];
        for v in &mut r_axis_invert {
            *v = r.u8()?;
        }

        Ok(AdcSettings {
            r_axis_min,
            r_axis_max,
            r_axis_smoothing,
            r_axis_to_button_low,
            r_axis_to_button_high,
            r_axis_invert,
        })
    }

    /// Reads firmware license information from the device.
    pub fn read_firmware_license(&self) -> Result<FirmwareLicense, WheelError> {
        let buf = self.get_feature_report(ReportType::FirmwareLicenseFeature as u8)?;
        let mut r = StructReader::new(&buf);
        Ok(FirmwareLicense {
            firmware_version: FirmwareVersion {
                release_type: r.u8()?,
                major: r.u8()?,
                minor: r.u8()?,
                patch: r.u8()?,
            },
            serial_key: [r.u32()?, r.u32()?, r.u32()?],
            device_id: [r.u32()?, r.u32()?, r.u32()?],
            is_registered: r.u8()?,
        })
    }

    /// Reads all settings groups from the device in sequence.
    pub fn read_all_settings(&self) -> Result<DeviceSettings, WheelError> {
        Ok(DeviceSettings {
            effects: self.read_effect_settings()?,
            hardware: self.read_hardware_settings()?,
            gpio: self.read_gpio_extension_settings()?,
            adc: self.read_adc_extension_settings()?,
        })
    }

    // -----------------------------------------------------------------------
    // Reading state
    // -----------------------------------------------------------------------

    /// Reads a single device state report (100 ms timeout).
    ///
    /// Returns `Ok(None)` when no report arrives within the timeout.
    /// The received state is cached and accessible via [`last_state`].
    pub fn read_state(&mut self) -> Result<Option<DeviceState>, WheelError> {
        // hidapi read buffer: [report_id (1)] + [report_data (64)]
        let mut buf = [0u8; REPORT_SIZE + 1];
        let n = self.device.read_timeout(&mut buf, 100)?;

        if n == 0 || buf[0] != ReportType::GenericInputOutput as u8 {
            return Ok(None);
        }

        // Parse from byte 1 (skipping report ID), mirroring DeviceStateTypeDef.
        let mut r = StructReader::new(&buf[1..]);

        let firmware_version = FirmwareVersion {
            release_type: r.u8()?,
            major: r.u8()?,
            minor: r.u8()?,
            patch: r.u8()?,
        };
        let is_registered = r.u8()?;
        let position = r.i16()?;
        let torque = r.i16()?;

        let position_degrees = self
            .cached_motion_range
            .map(|range| convert_position_to_degrees(position, range));

        let torque_normalized = normalize_torque(torque);

        let state = DeviceState {
            firmware_version,
            is_registered,
            position,
            torque,
            position_degrees,
            torque_normalized,
        };

        self.last_state = Some(state.clone());
        Ok(Some(state))
    }

    // -----------------------------------------------------------------------
    // Writing settings
    // -----------------------------------------------------------------------

    /// Writes a single setting value to the device.
    ///
    /// `value` is cast and clamped to the correct type for `field` using the
    /// same type map as the original TypeScript implementation.
    ///
    /// If `field` is [`SettingField::MotionRange`] the cached value is updated
    /// so that subsequent state reports include correct position-in-degrees.
    pub fn send_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: i64,
    ) -> Result<(), WheelError> {
        let buf = build_setting_buf(field, index, value);
        self.device.write(&buf)?;
        if field == SettingField::MotionRange {
            self.cached_motion_range = Some(value.clamp(0, 65535) as u16);
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Typed setting helpers (mirrors C++ API)
    // -----------------------------------------------------------------------

    /// Sends an `i8` setting report.
    pub fn send_i8_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: i8,
    ) -> Result<(), WheelError> {
        self.send_setting(field, index, value as i64)
    }

    /// Sends a `u8` setting report.
    pub fn send_u8_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: u8,
    ) -> Result<(), WheelError> {
        self.send_setting(field, index, value as i64)
    }

    /// Sends an `i16` setting report.
    pub fn send_i16_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: i16,
    ) -> Result<(), WheelError> {
        self.send_setting(field, index, value as i64)
    }

    /// Sends a `u16` setting report.
    pub fn send_u16_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: u16,
    ) -> Result<(), WheelError> {
        self.send_setting(field, index, value as i64)
    }

    /// Sends a `f32` setting report.
    pub fn send_float_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: f32,
    ) -> Result<(), WheelError> {
        // Route through the float path directly to avoid lossy i64 cast.
        let mut buf = [0u8; REPORT_SIZE + 1];
        buf[0] = ReportType::GenericInputOutput as u8;
        buf[1] = ReportData::DataSettingsFieldData as u8;
        buf[2] = field as u8;
        buf[3] = index;
        buf[4..8].copy_from_slice(&value.to_le_bytes());
        self.device.write(&buf)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Direct control
    // -----------------------------------------------------------------------

    /// Sends direct force feedback control to the device.
    ///
    /// Force values are clamped to their valid ranges before transmission.
    pub fn send_direct_control(&self, control: &DirectControl) -> Result<(), WheelError> {
        let buf = build_direct_control_buf(control);
        self.device.write(&buf)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Firmware activation
    // -----------------------------------------------------------------------

    /// Sends a firmware activation license key to the device.
    ///
    /// Expected format: `"XXXXXXXX-XXXXXXXX-XXXXXXXX"` (three 8-character
    /// hex segments separated by dashes).
    ///
    /// Returns immediately without sending if the device is already registered.
    pub fn send_firmware_activation(&self, license: &str) -> Result<(), WheelError> {
        if self
            .last_state
            .as_ref()
            .map(|s| s.is_registered == 1)
            .unwrap_or(false)
        {
            return Ok(());
        }

        let key = parse_license_key(license)?;

        // HID write buffer layout:
        // [0]    report ID   = 0xA3
        // [1]    report data = 0x13 (DataFirmwareActivationData)
        // [2–5]  chunk 0 as u32 LE
        // [6–9]  chunk 1 as u32 LE
        // [10–13] chunk 2 as u32 LE
        let mut buf = [0u8; REPORT_SIZE + 1];
        buf[0] = ReportType::GenericInputOutput as u8;
        buf[1] = ReportData::DataFirmwareActivationData as u8;
        for (i, &chunk) in key.iter().enumerate() {
            let offset = 2 + i * 4;
            buf[offset..offset + 4].copy_from_slice(&chunk.to_le_bytes());
        }

        self.device.write(&buf)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Commands
    // -----------------------------------------------------------------------

    /// Saves current settings to flash and reboots the controller.
    pub fn save_and_reboot(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandSaveSettings)
    }

    /// Reboots the controller without saving settings.
    pub fn reboot_controller(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandReboot)
    }

    /// Switches the device to DFU mode for firmware updates.
    pub fn switch_to_dfu(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandDfuMode)
    }

    /// Sets the current wheel position as the center point.
    pub fn reset_wheel_center(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandResetCenter)
    }

    // -----------------------------------------------------------------------
    // Event loop
    // -----------------------------------------------------------------------

    /// Blocking state-polling loop.
    ///
    /// Continuously reads device state reports and forwards each to
    /// `callback`. Return `true` from the callback to keep going, or `false`
    /// to stop cleanly.
    ///
    /// Any HID or parse error is propagated and terminates the loop.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ffbeast_wheel_api::WheelApi;
    /// let mut wheel = WheelApi::connect().unwrap();
    /// wheel.listen(|state| {
    ///     println!("pos {} torque {}", state.position, state.torque);
    ///     true
    /// }).unwrap();
    /// ```
    pub fn listen<F>(&mut self, mut callback: F) -> Result<(), WheelError>
    where
        F: FnMut(DeviceState) -> bool,
    {
        loop {
            match self.read_state()? {
                Some(state) => {
                    if !callback(state) {
                        return Ok(());
                    }
                }
                None => {} // Timeout — no report within 100 ms, keep polling.
            }
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn send_generic_command(&self, command: ReportData) -> Result<(), WheelError> {
        let mut buf = [0u8; REPORT_SIZE + 1];
        buf[0] = ReportType::GenericInputOutput as u8;
        buf[1] = command as u8;
        self.device.write(&buf)?;
        Ok(())
    }

    /// Sends a feature-report request and returns the payload bytes (report ID
    /// stripped, matching WebHID `receiveFeatureReport` behaviour).
    fn get_feature_report(&self, report_id: u8) -> Result<Vec<u8>, WheelError> {
        // hidapi convention: buf[0] = report ID on entry; returned data also
        // starts at buf[0]. Total buffer = 1 (report ID) + 64 (payload).
        let mut buf = vec![0u8; REPORT_SIZE + 1];
        buf[0] = report_id;
        let n = self.device.get_feature_report(&mut buf)?;
        if n < 2 {
            return Err(WheelError::BufferTooSmall {
                expected: 2,
                got: n,
            });
        }
        // Strip the report ID byte so callers see only the payload.
        Ok(buf[1..n].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // build_setting_buf
    // ------------------------------------------------------------------

    #[test]
    fn setting_buf_header_bytes() {
        let buf = build_setting_buf(SettingField::MotionRange, 0, 900);
        assert_eq!(buf[0], ReportType::GenericInputOutput as u8); // 0xA3
        assert_eq!(buf[1], ReportData::DataSettingsFieldData as u8); // 0x14
        assert_eq!(buf[2], SettingField::MotionRange as u8);
        assert_eq!(buf[3], 0); // index
    }

    #[test]
    fn setting_buf_u16_value_little_endian() {
        // MotionRange is Uint16; value 900 = 0x0384 → [0x84, 0x03]
        let buf = build_setting_buf(SettingField::MotionRange, 0, 900);
        assert_eq!(u16::from_le_bytes([buf[4], buf[5]]), 900);
    }

    #[test]
    fn setting_buf_i8_negative_value() {
        // DirectXConstantDirection is Int8; -1 encodes to 0xFF
        let buf = build_setting_buf(SettingField::DirectXConstantDirection, 0, -1);
        assert_eq!(buf[4] as i8, -1_i8);
    }

    #[test]
    fn setting_buf_u8_clamps_negative_to_zero() {
        // TotalEffectStrength is Uint8; -5 should clamp to 0
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, -5);
        assert_eq!(buf[4], 0);
    }

    #[test]
    fn setting_buf_u8_clamps_overflow_to_255() {
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, 999);
        assert_eq!(buf[4], 255);
    }

    #[test]
    fn setting_buf_index_is_stored() {
        let buf = build_setting_buf(SettingField::PinMode, 7, 1);
        assert_eq!(buf[3], 7);
    }

    #[test]
    fn setting_buf_tail_is_zero_padded() {
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, 42);
        // Only buf[4] is set for a u8 field; rest must be 0
        assert!(buf[5..].iter().all(|&b| b == 0));
    }

    // ------------------------------------------------------------------
    // build_direct_control_buf
    // ------------------------------------------------------------------

    #[test]
    fn direct_control_buf_header() {
        let buf = build_direct_control_buf(&DirectControl::default());
        assert_eq!(buf[0], ReportType::GenericInputOutput as u8);
        assert_eq!(buf[1], ReportData::DataOverrideData as u8);
    }

    #[test]
    fn direct_control_buf_forces_little_endian() {
        let ctrl = DirectControl {
            spring_force: 1_000,
            constant_force: -500,
            periodic_force: 0,
            force_drop: 25,
        };
        let buf = build_direct_control_buf(&ctrl);
        assert_eq!(i16::from_le_bytes([buf[2], buf[3]]), 1_000);
        assert_eq!(i16::from_le_bytes([buf[4], buf[5]]), -500);
        assert_eq!(i16::from_le_bytes([buf[6], buf[7]]), 0);
        assert_eq!(buf[8], 25);
    }

    #[test]
    fn direct_control_buf_clamps_forces() {
        let ctrl = DirectControl {
            spring_force: 30_000,   // > 10 000
            constant_force: -30_000, // < -10 000
            periodic_force: 0,
            force_drop: 200, // > 100
        };
        let buf = build_direct_control_buf(&ctrl);
        assert_eq!(i16::from_le_bytes([buf[2], buf[3]]), 10_000);
        assert_eq!(i16::from_le_bytes([buf[4], buf[5]]), -10_000);
        assert_eq!(buf[8], 100);
    }

    // ------------------------------------------------------------------
    // parse_license_key
    // ------------------------------------------------------------------

    #[test]
    fn license_valid_parses_three_u32s() {
        let key = parse_license_key("AABBCCDD-11223344-DEADBEEF").unwrap();
        assert_eq!(key[0], 0xAABB_CCDD);
        assert_eq!(key[1], 0x1122_3344);
        assert_eq!(key[2], 0xDEAD_BEEF);
    }

    #[test]
    fn license_lowercase_hex_accepted() {
        let key = parse_license_key("aabbccdd-11223344-deadbeef").unwrap();
        assert_eq!(key[0], 0xAABB_CCDD);
    }

    #[test]
    fn license_with_leading_trailing_whitespace_accepted() {
        assert!(parse_license_key("  AABBCCDD-11223344-DEADBEEF  ").is_ok());
    }

    #[test]
    fn license_wrong_segment_count_is_err() {
        assert!(parse_license_key("AABBCCDD-11223344").is_err());
        assert!(parse_license_key("AABBCCDD-11223344-DEADBEEF-00000000").is_err());
    }

    #[test]
    fn license_short_segment_is_err() {
        assert!(parse_license_key("AABB-11223344-DEADBEEF").is_err());
    }

    #[test]
    fn license_non_hex_chars_are_err() {
        assert!(parse_license_key("GGHHIIJJ-11223344-DEADBEEF").is_err());
    }
}
