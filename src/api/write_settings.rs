//! `WheelApi` write-settings methods: generic and typed setting senders.

use crate::constants::REPORT_SIZE;
use crate::enums::{ReportData, ReportType, SettingField};
use crate::error::WheelError;
use crate::protocol::build_setting_buf;

use super::WheelApi;

impl WheelApi {
    /// Writes a single setting value to the device.
    ///
    /// `value` is cast and clamped to the correct wire type for `field` using
    /// the same type map as the original TypeScript and C++ implementations.
    ///
    /// If `field` is [`SettingField::MotionRange`] the cached motion range is
    /// updated so that subsequent state reports include correct position-in-degrees.
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

    /// Sends an `f32` setting report.
    ///
    /// Routes through the float path directly to avoid lossy `f32 → i64` cast.
    pub fn send_float_setting(
        &mut self,
        field: SettingField,
        index: u8,
        value: f32,
    ) -> Result<(), WheelError> {
        let mut buf = [0u8; REPORT_SIZE + 1];
        buf[0] = ReportType::GenericInputOutput as u8;
        buf[1] = ReportData::DataSettingsFieldData as u8;
        buf[2] = field as u8;
        buf[3] = index;
        buf[4..8].copy_from_slice(&value.to_le_bytes());
        self.device.write(&buf)?;
        Ok(())
    }
}
