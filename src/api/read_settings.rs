//! `WheelApi` read-settings methods: individual settings groups and combined snapshot.

use crate::error::WheelError;
use crate::enums::ReportType;
use crate::protocol::StructReader;
use crate::types::{AdcSettings, DeviceSettings, EffectSettings, FirmwareLicense, FirmwareVersion, GpioSettings, HardwareSettings};

use super::WheelApi;

impl WheelApi {
    /// Reads effect / force-feedback settings from the device.
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

    /// Reads hardware / motor settings from the device.
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

    /// Reads GPIO extension pin and SPI settings from the device.
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

    /// Reads ADC extension axis settings from the device.
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

    /// Reads firmware license and registration information from the device.
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

    /// Reads all settings groups from the device and returns a combined snapshot.
    pub fn read_all_settings(&self) -> Result<DeviceSettings, WheelError> {
        Ok(DeviceSettings {
            effects: self.read_effect_settings()?,
            hardware: self.read_hardware_settings()?,
            gpio: self.read_gpio_extension_settings()?,
            adc: self.read_adc_extension_settings()?,
        })
    }
}
