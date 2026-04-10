//! Public data types for device state, settings, firmware, and control.

pub mod control;
pub mod firmware;
pub mod settings;
pub mod state;

pub use control::DirectControl;
pub use firmware::{FirmwareLicense, FirmwareVersion};
pub use settings::{AdcSettings, DeviceSettings, EffectSettings, GpioSettings, HardwareSettings};
pub use state::DeviceState;
