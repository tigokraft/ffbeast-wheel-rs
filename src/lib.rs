//! # ffbeast-wheel-api
//!
//! HID-based Rust library for communicating with the **FFBeast wheel controller**.
//!
//! This is a full port of the original TypeScript WebHID API
//! (`@shubham0x13/ffbeast-wheel-webhid-api`) using native HID via the
//! [`hidapi`] crate, mirroring the C++ reference implementation's struct
//! layout and report format.
//!
//! ## Quick start
//!
//! ```no_run
//! use ffbeast_wheel_api::WheelApi;
//!
//! let mut wheel = WheelApi::connect().expect("connect failed");
//!
//! let settings = wheel.read_all_settings().expect("read settings failed");
//! println!("motion range: {} °", settings.effects.motion_range);
//!
//! wheel.listen(|state| {
//!     println!("pos={:6}  torque={:6}  deg={:?}", state.position, state.torque, state.position_degrees);
//!     true
//! }).expect("listen failed");
//! ```

#![warn(missing_docs)]

/// USB VID/PID and HID report size constants.
pub mod constants;
/// Enums and the field-type map for the device protocol.
pub mod enums;
/// Error type returned by all fallible API calls.
pub mod error;
/// Data structs that model device state and settings.
pub mod types;
/// Byte-level parsing utilities and unit-conversion helpers.
pub mod utils;
/// Main [`WheelApi`] implementation.
pub mod wheel_api;

// Convenience re-exports — mirror the flat export surface of the TypeScript package.
pub use enums::{
    AmplifierGain, ButtonMode, ExtensionMode, FieldType, PinMode, ReportData, ReportType,
    SettingField, SpiLatchMode, SpiMode,
};
pub use error::WheelError;
pub use types::{
    AdcSettings, DeviceSettings, DeviceState, DirectControl, EffectSettings, FirmwareLicense,
    FirmwareVersion, GpioSettings, HardwareSettings,
};
pub use utils::{convert_position_to_degrees, normalize_torque};
pub use wheel_api::WheelApi;

