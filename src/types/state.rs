//! Real-time device state received from the wheel controller.

use crate::types::firmware::FirmwareVersion;

/// Real-time device state received from the wheel controller.
///
/// This is the parsed form of the raw interrupt HID report (report ID `0xA3`).
#[derive(Debug, Clone)]
pub struct DeviceState {
    /// Firmware version reported in each interrupt packet.
    pub firmware_version: FirmwareVersion,
    /// Registration status (`0` = unregistered, `1` = registered).
    pub is_registered: u8,
    /// Raw wheel position. Range: roughly −10 000 to +10 000.
    pub position: i16,
    /// Raw torque currently being output. Range: −10 000 to +10 000.
    pub torque: i16,
    /// Wheel position in degrees based on the active motion range.
    /// `None` if the motion range has not yet been cached.
    pub position_degrees: Option<f64>,
    /// Torque normalised to −100.0 to +100.0.
    /// Positive = right (CW), negative = left (CCW).
    pub torque_normalized: f64,
}
