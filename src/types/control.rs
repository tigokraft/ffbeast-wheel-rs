//! Direct force-feedback control override packet.

/// Low-level force-feedback control packet sent directly to the motor driver.
///
/// All force values use a normalised range of −10 000 to +10 000
/// (i.e. −1.0 to +1.0 with four decimal digits of precision).
#[derive(Debug, Clone, Default)]
pub struct DirectControl {
    /// Spring force acting opposite to wheel rotation. Range: −10 000 to +10 000.
    pub spring_force: i16,
    /// Constant force moving the wheel in a fixed direction. Range: −10 000 to +10 000.
    pub constant_force: i16,
    /// Periodic effect force (sine/triangle/etc.). Range: −10 000 to +10 000.
    pub periodic_force: i16,
    /// Global force scaling factor (inverse).
    /// Formula: `TotalForce = InitialForce × (1 − force_drop / 100)`.
    /// Range: 0 to 100.
    pub force_drop: u8,
}
