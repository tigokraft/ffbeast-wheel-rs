/// USB Vendor ID for FFBeast devices.
pub const VID: u16 = 1115;

/// USB Product ID for FFBeast wheel controllers.
pub const PID: u16 = 22999;

/// Size of HID reports in bytes.
pub const REPORT_SIZE: usize = 64;

/// Maximum raw position value from the device.
/// Used for converting raw position to degrees.
pub const RAW_POSITION_MAX: f64 = 10000.0;

/// Maximum raw torque value from the device.
/// Used for normalizing torque to [-100, 100] range.
pub const RAW_TORQUE_MAX: f64 = 10000.0;
