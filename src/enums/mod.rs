//! Enumerations for GPIO, hardware, SPI, report types, and settings fields.

pub mod gpio;
pub mod hardware;
pub mod report;
pub mod settings;
pub mod spi;

pub use gpio::{ButtonMode, ExtensionMode, PinMode};
pub use hardware::AmplifierGain;
pub use report::{ReportData, ReportType};
pub use settings::{field_type_for, FieldType, SettingField};
pub use spi::{SpiLatchMode, SpiMode};
