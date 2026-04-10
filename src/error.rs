//! Error types for `ffbeast-wheel-api`.
use thiserror::Error;

/// All errors that can be returned by [`crate::WheelApi`].
#[derive(Debug, Error)]
pub enum WheelError {
    /// No FFBeast wheel was found in the HID device list.
    #[error("Device not found")]
    DeviceNotFound,

    /// An underlying HID communication error.
    #[error("HID error: {0}")]
    Hid(#[from] hidapi::HidError),

    /// A feature report was shorter than expected.
    #[error("Parse error: buffer too small (expected at least {expected} bytes, got {got})")]
    BufferTooSmall {
        /// Minimum number of bytes required.
        expected: usize,
        /// Actual number of bytes available.
        got: usize,
    },

    /// The license key string did not match the expected format.
    #[error("Invalid license format: {0}")]
    InvalidLicense(String),
}
