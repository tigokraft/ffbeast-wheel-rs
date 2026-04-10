//! Current-sense amplifier gain setting.

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
