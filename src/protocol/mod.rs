//! Wire-protocol helpers: byte reader, math conversions, HID buffer builders, license parser.

pub mod buffers;
pub mod license;
pub mod math;
pub mod reader;

pub use math::{convert_position_to_degrees, normalize_torque};
pub use reader::StructReader;
pub(crate) use buffers::{build_direct_control_buf, build_setting_buf};
pub(crate) use license::parse_license_key;
