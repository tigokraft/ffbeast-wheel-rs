//! `WheelApi` direct force-feedback control and firmware activation.

use crate::constants::REPORT_SIZE;
use crate::enums::{ReportData, ReportType};
use crate::error::WheelError;
use crate::protocol::{build_direct_control_buf, parse_license_key};
use crate::types::DirectControl;

use super::WheelApi;

impl WheelApi {
    /// Sends a direct force-feedback control packet to the device.
    ///
    /// Force values are clamped to their valid ranges before transmission.
    /// Use this for low-latency custom force output bypassing the PID loop.
    pub fn send_direct_control(&self, control: &DirectControl) -> Result<(), WheelError> {
        let buf = build_direct_control_buf(control);
        self.device.write(&buf)?;
        Ok(())
    }

    /// Sends a firmware activation license key to the device.
    ///
    /// Expected format: `"XXXXXXXX-XXXXXXXX-XXXXXXXX"` (three 8-character hex
    /// segments separated by hyphens).
    ///
    /// Returns immediately without sending if the device is already registered.
    ///
    /// # Errors
    ///
    /// Returns [`crate::WheelError::InvalidLicense`] if the key format is invalid.
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
        // [0]    report ID    = 0xA3
        // [1]    report data  = 0x13 (DataFirmwareActivationData)
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
}
