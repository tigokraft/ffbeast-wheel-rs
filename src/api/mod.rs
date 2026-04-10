//! `WheelApi` struct definition, accessors, private HID helpers, and submodule declarations.

mod commands;
mod connect;
mod control;
mod read_settings;
mod state;
mod write_settings;

use crate::constants::REPORT_SIZE;
use crate::error::WheelError;
use crate::protocol::{convert_position_to_degrees, normalize_torque};
use crate::types::DeviceState;

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
    pub(super) device: hidapi::HidDevice,
    pub(super) cached_motion_range: Option<u16>,
    pub(super) last_state: Option<DeviceState>,
}

impl WheelApi {
    // -----------------------------------------------------------------------
    // Computed accessors
    // -----------------------------------------------------------------------

    /// Last device state received via [`Self::read_state`] or [`Self::listen`].
    pub fn last_state(&self) -> Option<&DeviceState> {
        self.last_state.as_ref()
    }

    /// Motion range (degrees) cached at connect time and updated by
    /// [`crate::api::write_settings`] when [`crate::enums::SettingField::MotionRange`] is written.
    pub fn cached_motion_range(&self) -> Option<u16> {
        self.cached_motion_range
    }

    /// Current wheel position in degrees, or `None` if unavailable.
    pub fn position_degrees(&self) -> Option<f64> {
        let state = self.last_state.as_ref()?;
        let range = self.cached_motion_range?;
        Some(convert_position_to_degrees(state.position, range))
    }

    /// Current torque normalised to `[−100.0, 100.0]`, or `None` if unavailable.
    pub fn torque_normalized(&self) -> Option<f64> {
        Some(normalize_torque(self.last_state.as_ref()?.torque))
    }

    /// Firmware version string (e.g. `"1.2.3"`), or `None` if unavailable.
    pub fn firmware_version(&self) -> Option<String> {
        let v = &self.last_state.as_ref()?.firmware_version;
        Some(format!("{}.{}.{}", v.major, v.minor, v.patch))
    }

    /// Firmware release type byte, or `None` if unavailable.
    pub fn firmware_release_type(&self) -> Option<u8> {
        Some(self.last_state.as_ref()?.firmware_version.release_type)
    }

    // -----------------------------------------------------------------------
    // Crate-private HID helpers shared across all api/ submodules
    // -----------------------------------------------------------------------

    /// Sends a feature-report request and returns the payload bytes (report ID
    /// stripped, matching WebHID `receiveFeatureReport` behaviour).
    pub(super) fn get_feature_report(&self, report_id: u8) -> Result<Vec<u8>, WheelError> {
        let mut buf = vec![0u8; REPORT_SIZE + 1];
        buf[0] = report_id;
        let n = self.device.get_feature_report(&mut buf)?;
        if n < 2 {
            return Err(WheelError::BufferTooSmall {
                expected: 2,
                got: n,
            });
        }
        Ok(buf[1..n].to_vec())
    }

    /// Sends a zero-payload generic command report.
    pub(super) fn send_generic_command(
        &self,
        command: crate::enums::ReportData,
    ) -> Result<(), WheelError> {
        let mut buf = [0u8; REPORT_SIZE + 1];
        buf[0] = crate::enums::ReportType::GenericInputOutput as u8;
        buf[1] = command as u8;
        self.device.write(&buf)?;
        Ok(())
    }
}
