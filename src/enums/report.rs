//! HID report type and sub-type identifiers.

/// HID report sub-type byte sent as the first payload byte of a `GenericInputOutput` report.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportData {
    /// Reboot the controller immediately (no save).
    CommandReboot = 0x01,
    /// Save current settings to flash, then reboot.
    CommandSaveSettings = 0x02,
    /// Enter DFU (firmware update) bootloader mode.
    CommandDfuMode = 0x03,
    /// Set current wheel position as the center reference.
    CommandResetCenter = 0x04,
    /// Direct force override packet (`DataOverrideTypeDef`).
    DataOverrideData = 0x10,
    /// Firmware activation / license key packet.
    DataFirmwareActivationData = 0x13,
    /// Single setting field write packet (`DataSettingsFieldTypeDef`).
    DataSettingsFieldData = 0x14,
}

/// HID report type identifiers.
///
/// Note: several values are shared across input/output/feature categories as
/// in the original C++ and TypeScript sources.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    /// Standard HID joystick input report.
    JoystickInput = 0x01,
    /// PID set-effect output report.
    SetEffect = 0x11,
    /// PID set-envelope output report.
    SetEnvelope = 0x12,
    /// PID set-condition output report.
    SetCondition = 0x13,
    /// PID set-periodic output report.
    SetPeriodic = 0x14,
    /// PID set-constant-force output report.
    SetConstantForce = 0x15,
    /// PID set-ramp-force output report.
    SetRampForce = 0x16,
    /// PID effect-operation output report.
    EffectOperation = 0x1a,
    /// PID block-free output report.
    PIDBlockFree = 0x1b,
    /// PID device-control output report.
    PIDDeviceControl = 0x1c,
    /// PID device-gain output report.
    DeviceGain = 0x1d,
    /// Hardware settings feature report (`HardwareSettingsTypeDef`).
    HardwareSettingsFeature = 0x21,
    /// Effect settings feature report (`EffectSettingsTypeDef`).
    EffectSettingsFeature = 0x22,
    /// Firmware license feature report (`FirmwareLicenseTypeDef`).
    FirmwareLicenseFeature = 0x25,
    /// GPIO extension settings feature report (`GpioExtensionSettingsTypeDef`).
    GpioSettingsFeature = 0xa1,
    /// ADC extension settings feature report (`AdcExtensionSettingsTypeDef`).
    AdcSettingsFeature = 0xa2,
    /// Vendor generic input/output report used for all custom commands.
    GenericInputOutput = 0xa3,
}
