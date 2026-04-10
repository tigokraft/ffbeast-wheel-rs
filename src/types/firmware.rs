//! Firmware version and license key types.

/// Firmware version reported by the device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareVersion {
    /// Release type byte (`0` = release, non-zero = pre-release/debug).
    pub release_type: u8,
    /// Year of the release.
    pub major: u8,
    /// Minor version (incremented when companion app update is needed).
    pub minor: u8,
    /// Patch version (incremented on each patch within the same version).
    pub patch: u8,
}

/// Firmware license information read from the device.
#[derive(Debug, Clone)]
pub struct FirmwareLicense {
    /// Version of the running firmware.
    pub firmware_version: FirmwareVersion,
    /// Serial key components (3 × 32-bit values).
    pub serial_key: [u32; 3],
    /// Device ID components (3 × 32-bit values).
    pub device_id: [u32; 3],
    /// Registration status (`0` = unregistered, `1` = registered).
    pub is_registered: u8,
}
