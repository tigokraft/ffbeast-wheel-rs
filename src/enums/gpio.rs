//! GPIO pin, button, and extension-mode enums.

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
