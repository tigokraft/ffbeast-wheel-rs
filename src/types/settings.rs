//! Device settings structs: effect, hardware, ADC, GPIO, and combined.

/// Force-feedback effect parameters stored on the device.
#[derive(Debug, Clone)]
pub struct EffectSettings {
    /// Motion range in degrees.
    pub motion_range: u16,
    /// Static dampening strength (0 to 100%).
    pub static_dampening_strength: u16,
    /// Soft stop dampening strength (0 to 100%).
    pub soft_stop_dampening_strength: u16,
    /// Total effect strength (0 to 100%).
    pub total_effect_strength: u8,
    /// Integrated spring strength (0 to 100%).
    pub integrated_spring_strength: u8,
    /// Soft stop range in degrees (added on top of motion range).
    pub soft_stop_range: u8,
    /// Soft stop strength (0 to 100%).
    pub soft_stop_strength: u8,
    /// DirectX constant force direction (`-1` or `+1`).
    pub direct_x_constant_direction: i8,
    /// DirectX spring strength (0 to 100%).
    pub direct_x_spring_strength: u8,
    /// DirectX constant strength (0 to 100%).
    pub direct_x_constant_strength: u8,
    /// DirectX periodic strength (0 to 100%).
    pub direct_x_periodic_strength: u8,
}

/// Hardware settings stored on the device.
#[derive(Debug, Clone)]
pub struct HardwareSettings {
    /// Encoder counts per revolution (CPR).
    pub encoder_cpr: u16,
    /// Integral gain (I-Gain) for PID.
    pub integral_gain: u16,
    /// Proportional gain (P-Gain) for PID.
    pub proportional_gain: u8,
    /// Force feedback enabled (`0` = Disabled, `1` = Enabled).
    pub force_enabled: u8,
    /// Debug torque output enabled (`0` = Disabled, `1` = Enabled).
    pub debug_torque: u8,
    /// Amplifier gain setting (raw byte). See [`crate::AmplifierGain`].
    pub amplifier_gain: u8,
    /// Calibration magnitude (0 to 100%).
    pub calibration_magnitude: u8,
    /// Calibration speed (0 to 100%).
    pub calibration_speed: u8,
    /// Power limit (0 to 100%).
    pub power_limit: u8,
    /// Braking limit (0 to 100%).
    pub braking_limit: u8,
    /// Position smoothing (0 to 100%).
    pub position_smoothing: u8,
    /// Speed buffer size.
    pub speed_buffer_size: u8,
    /// Encoder direction multiplier (`-1` or `+1`).
    pub encoder_direction: i8,
    /// Force direction multiplier (`-1` or `+1`).
    pub force_direction: i8,
    /// Number of motor pole pairs.
    pub pole_pairs: u8,
}

/// ADC extension settings stored on the device.
#[derive(Debug, Clone)]
pub struct AdcSettings {
    /// Minimum raw values for the 3 analog axes.
    pub r_axis_min: [u16; 3],
    /// Maximum raw values for the 3 analog axes.
    pub r_axis_max: [u16; 3],
    /// Axis smoothing factor per axis. Divide by 100 to get normalised ratio (0..1).
    pub r_axis_smoothing: [u8; 3],
    /// Point in % where "Button Low" is triggered per axis. `0` = disabled.
    pub r_axis_to_button_low: [u8; 3],
    /// Point in % where "Button High" is triggered per axis. `100` = disabled.
    pub r_axis_to_button_high: [u8; 3],
    /// Axis inversion flags per axis (`0` or `1`).
    pub r_axis_invert: [u8; 3],
}

/// GPIO extension settings stored on the device.
///
/// All fields are stored as raw `u8` values on the wire. Use the
/// enum types in [`crate::enums`] for human-readable interpretation.
#[derive(Debug, Clone)]
pub struct GpioSettings {
    /// Extension mode. See [`crate::ExtensionMode`].
    pub extension_mode: u8,
    /// Pin mode configuration for 10 pins. See [`crate::PinMode`].
    pub pin_mode: [u8; 10],
    /// Button mode configuration for 32 buttons. See [`crate::ButtonMode`].
    pub button_mode: [u8; 32],
    /// SPI communication mode. See [`crate::SpiMode`].
    pub spi_mode: u8,
    /// SPI latch mode. See [`crate::SpiLatchMode`].
    pub spi_latch_mode: u8,
    /// SPI latch delay in microseconds.
    pub spi_latch_delay: u8,
    /// SPI clock pulse length in microseconds.
    pub spi_clk_pulse_length: u8,
}

/// Aggregated settings object containing all configuration groups.
#[derive(Debug, Clone)]
pub struct DeviceSettings {
    /// Effect / force-feedback settings.
    pub effects: EffectSettings,
    /// Motor / hardware settings.
    pub hardware: HardwareSettings,
    /// GPIO extension settings.
    pub gpio: GpioSettings,
    /// ADC extension settings.
    pub adc: AdcSettings,
}
