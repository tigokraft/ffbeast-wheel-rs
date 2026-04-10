/// Controls which extension protocol is active on the GPIO header.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionMode {
    /// No extension active; GPIO pins used in standalone mode.
    None = 0,
    /// Custom SPI/ADC extension attached.
    Custom = 1,
}

/// Function assigned to an individual GPIO header pin.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    /// Pin disabled / not used.
    None = 0,
    /// General-purpose digital input (button).
    Gpio = 1,
    /// Analog axis input (ADC).
    Analog = 2,
    /// SPI chip-select output.
    SpiCs = 3,
    /// SPI clock output.
    SpiSck = 4,
    /// SPI MISO input.
    SpiMiso = 5,
    /// Active-high input that enables force-feedback output while held.
    EnableEffects = 6,
    /// Active-high input that resets the wheel center point.
    CenterReset = 7,
    /// PWM output for braking.
    BrakingPwm = 8,
    /// LED output that mirrors the force-feedback active state.
    EffectLed = 9,
    /// Active-high input that reboots the controller.
    Reboot = 10,
}

/// Logic applied to a digital button input.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonMode {
    /// Button disabled.
    None = 0,
    /// Active-high (pressed = logic 1).
    Normal = 1,
    /// Active-low (pressed = logic 0).
    Inverted = 2,
    /// Not implemented in firmware.
    Pulse = 3,
    /// Not implemented in firmware.
    PulseInverted = 4,
}

/// Current-sense amplifier gain setting.
///
/// Lower gain → higher maximum detectable current → higher peak torque.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmplifierGain {
    /// ×80 gain (lowest current range, highest sensitivity).
    Gain80 = 0,
    /// ×40 gain.
    Gain40 = 1,
    /// ×20 gain.
    Gain20 = 2,
    /// ×10 gain (highest current range, lowest sensitivity).
    Gain10 = 3,
}

/// SPI clock/latch behaviour modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiMode {
    /// First bit output immediately when CS activates. READ-CLK_UP-DELAY-CLK_DOWN-DELAY
    Mode0 = 0,
    /// First bit output on first clock edge after CS activates. CLK_UP-DELAY-READ-CLK_DOWN-DELAY
    Mode1 = 1,
    /// First bit output immediately when CS activates. READ-CLK_DOWN-DELAY-CLK_UP-DELAY
    Mode2 = 2,
    /// First bit output on first clock edge after CS activates. CLK_DOWN-DELAY-READ-CLK_UP-DELAY
    Mode3 = 3,
}

/// Polarity of the SPI chip-select (nCS) latch pulse.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiLatchMode {
    /// nCS goes UP for triggering SPI latch.
    LatchUp = 0,
    /// nCS goes DOWN for triggering SPI latch.
    LatchDown = 1,
}

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

/// Device setting field identifiers (mirrors `SettingsFieldEnum` in C++).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingField {
    /// DirectX constant force direction multiplier (`-1` or `+1`).
    DirectXConstantDirection = 0,
    /// DirectX spring strength (0–100%).
    DirectXSpringStrength = 1,
    /// DirectX constant strength (0–100%).
    DirectXConstantStrength = 2,
    /// DirectX periodic strength (0–100%).
    DirectXPeriodicStrength = 3,
    /// Overall effect strength scaling (0–100%).
    TotalEffectStrength = 4,
    /// Wheel motion range in degrees (e.g. 900 for a 900 ° range).
    MotionRange = 5,
    /// Soft-stop spring strength (0–100%).
    SoftStopStrength = 6,
    /// Extra angular range beyond `MotionRange` used for soft-stop (degrees).
    SoftStopRange = 7,
    /// Passive damping applied at all times (0–100%).
    StaticDampeningStrength = 8,
    /// Damping strength inside the soft-stop zone (0–100%).
    SoftStopDampeningStrength = 9,
    /// Master force-feedback enable (`0` = off, `1` = on).
    ForceEnabled = 11,
    /// Constant debug torque output (used for tuning, 0–100%).
    DebugTorque = 12,
    /// Current-sense amplifier gain. See [`crate::AmplifierGain`].
    AmplifierGain = 13,
    /// Motor calibration sequence magnitude (0–100%).
    CalibrationMagnitude = 15,
    /// Motor calibration sequence speed (0–100%).
    CalibrationSpeed = 16,
    /// Overall power limit (0–100%).
    PowerLimit = 17,
    /// Regenerative braking limit (0–100%).
    BrakingLimit = 18,
    /// Position IIR smoothing factor (0–100).
    PositionSmoothing = 19,
    /// Speed estimation buffer length.
    SpeedBufferSize = 20,
    /// Encoder direction multiplier (`-1` or `+1`).
    EncoderDirection = 21,
    /// Motor force direction multiplier (`-1` or `+1`).
    ForceDirection = 22,
    /// Number of motor pole-pairs.
    PolePairs = 23,
    /// Encoder counts per revolution.
    EncoderCPR = 24,
    /// PID proportional gain.
    PGain = 25,
    /// PID integral gain.
    IGain = 26,
    /// GPIO extension mode. See [`crate::ExtensionMode`].
    ExtensionMode = 27,
    /// GPIO pin function (indexed per pin). See [`crate::PinMode`].
    PinMode = 28,
    /// GPIO button logic (indexed per button). See [`crate::ButtonMode`].
    ButtonMode = 29,
    /// SPI communication mode. See [`crate::SpiMode`].
    SpiMode = 30,
    /// SPI CS latch polarity. See [`crate::SpiLatchMode`].
    SpiLatchMode = 31,
    /// SPI CS latch delay in microseconds.
    SpiLatchDelay = 32,
    /// SPI clock pulse length in microseconds.
    SpiClkPulseLength = 33,
    /// ADC axis minimum dead-zone (raw units, indexed per axis).
    AdcMinDeadZone = 34,
    /// ADC axis maximum dead-zone (raw units, indexed per axis).
    AdcMaxDeadZone = 35,
    /// Raw % threshold at which the ADC axis triggers its low virtual button (indexed per axis).
    AdcToButtonLow = 36,
    /// Raw % threshold at which the ADC axis triggers its high virtual button (indexed per axis).
    AdcToButtonHigh = 37,
    /// ADC axis IIR smoothing factor (indexed per axis).
    AdcSmoothing = 38,
    /// ADC axis inversion flag (`0` or `1`, indexed per axis).
    AdcInvert = 39,
    /// Auto-reset center when the encoder Z-pulse fires (`0` or `1`).
    ResetCenterOnZ0 = 41,
    /// Integrated (position-proportional) spring strength (0–100%).
    IntegratedSpringStrength = 43,
}

/// Binary field type identifiers for encoding setting values in HID reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// Signed 8-bit integer.
    Int8,
    /// Unsigned 8-bit integer.
    Uint8,
    /// Signed 16-bit little-endian integer.
    Int16,
    /// Unsigned 16-bit little-endian integer.
    Uint16,
    /// Signed 32-bit little-endian integer.
    Int32,
    /// Unsigned 32-bit little-endian integer.
    Uint32,
    /// 32-bit IEEE 754 little-endian float.
    Float32,
}

/// Returns the [`FieldType`] for a given [`SettingField`].
///
/// Based on the packed struct definitions in the original C++ `wheel_api.h`.
pub fn field_type_for(field: SettingField) -> FieldType {
    match field {
        // EffectSettings
        SettingField::MotionRange => FieldType::Uint16,
        SettingField::StaticDampeningStrength => FieldType::Uint16,
        SettingField::SoftStopDampeningStrength => FieldType::Uint16,
        SettingField::TotalEffectStrength => FieldType::Uint8,
        SettingField::IntegratedSpringStrength => FieldType::Uint8,
        SettingField::SoftStopRange => FieldType::Uint8,
        SettingField::SoftStopStrength => FieldType::Uint8,
        SettingField::DirectXConstantDirection => FieldType::Int8,
        SettingField::DirectXSpringStrength => FieldType::Uint8,
        SettingField::DirectXConstantStrength => FieldType::Uint8,
        SettingField::DirectXPeriodicStrength => FieldType::Uint8,

        // HardwareSettings
        SettingField::EncoderCPR => FieldType::Uint16,
        SettingField::IGain => FieldType::Uint16,
        SettingField::PGain => FieldType::Uint8,
        SettingField::ForceEnabled => FieldType::Uint8,
        SettingField::DebugTorque => FieldType::Uint8,
        SettingField::AmplifierGain => FieldType::Uint8,
        SettingField::CalibrationMagnitude => FieldType::Uint8,
        SettingField::CalibrationSpeed => FieldType::Uint8,
        SettingField::PowerLimit => FieldType::Uint8,
        SettingField::BrakingLimit => FieldType::Uint8,
        SettingField::PositionSmoothing => FieldType::Uint8,
        SettingField::SpeedBufferSize => FieldType::Uint8,
        SettingField::EncoderDirection => FieldType::Int8,
        SettingField::ForceDirection => FieldType::Int8,
        SettingField::PolePairs => FieldType::Uint8,

        // GpioExtensionSettings
        SettingField::ExtensionMode => FieldType::Uint8,
        SettingField::PinMode => FieldType::Uint8,
        SettingField::ButtonMode => FieldType::Uint8,
        SettingField::SpiMode => FieldType::Uint8,
        SettingField::SpiLatchMode => FieldType::Uint8,
        SettingField::SpiLatchDelay => FieldType::Uint8,
        SettingField::SpiClkPulseLength => FieldType::Uint8,

        // AdcExtensionSettings
        SettingField::AdcSmoothing => FieldType::Uint8,
        SettingField::AdcToButtonLow => FieldType::Uint8,
        SettingField::AdcToButtonHigh => FieldType::Uint8,
        SettingField::AdcInvert => FieldType::Uint8,
        SettingField::AdcMinDeadZone => FieldType::Uint16,
        SettingField::AdcMaxDeadZone => FieldType::Uint16,

        SettingField::ResetCenterOnZ0 => FieldType::Uint8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_range_is_u16() {
        assert_eq!(field_type_for(SettingField::MotionRange), FieldType::Uint16);
    }

    #[test]
    fn constant_direction_is_i8() {
        assert_eq!(
            field_type_for(SettingField::DirectXConstantDirection),
            FieldType::Int8
        );
    }

    #[test]
    fn encoder_cpr_is_u16() {
        assert_eq!(field_type_for(SettingField::EncoderCPR), FieldType::Uint16);
    }

    #[test]
    fn total_effect_strength_is_u8() {
        assert_eq!(
            field_type_for(SettingField::TotalEffectStrength),
            FieldType::Uint8
        );
    }

    #[test]
    fn encoder_direction_is_i8() {
        assert_eq!(
            field_type_for(SettingField::EncoderDirection),
            FieldType::Int8
        );
    }

    #[test]
    fn adc_dead_zone_is_u16() {
        assert_eq!(
            field_type_for(SettingField::AdcMinDeadZone),
            FieldType::Uint16
        );
        assert_eq!(
            field_type_for(SettingField::AdcMaxDeadZone),
            FieldType::Uint16
        );
    }
}
