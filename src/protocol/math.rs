//! Mathematical helpers for converting raw sensor values.

use crate::constants::{RAW_POSITION_MAX, RAW_TORQUE_MAX};

/// Converts a raw encoder position to degrees given the configured motion range.
///
/// Formula: `(raw_position × motion_range) / (2 × RAW_POSITION_MAX)`
///
/// The result is centred at 0 — positive degrees = right of centre, negative = left.
pub fn convert_position_to_degrees(raw_position: i16, motion_range: u16) -> f64 {
    (raw_position as f64 * motion_range as f64) / (2.0 * RAW_POSITION_MAX)
}

/// Normalises a raw torque value to the `[−100.0, 100.0]` range.
///
/// Positive values correspond to clockwise (right) torque, negative to
/// counter-clockwise (left) torque.
pub fn normalize_torque(raw_torque: i16) -> f64 {
    (raw_torque as f64 * 100.0) / RAW_TORQUE_MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_degrees_center_is_zero() {
        assert_eq!(convert_position_to_degrees(0, 900), 0.0);
    }

    #[test]
    fn position_degrees_max_is_half_range() {
        // raw max (10 000) with 900 ° range → 450 °
        assert_eq!(convert_position_to_degrees(10_000, 900), 450.0);
    }

    #[test]
    fn position_degrees_min_is_negative_half_range() {
        assert_eq!(convert_position_to_degrees(-10_000, 900), -450.0);
    }

    #[test]
    fn position_degrees_scales_with_range() {
        let deg_900 = convert_position_to_degrees(5_000, 900);
        let deg_1800 = convert_position_to_degrees(5_000, 1800);
        assert!((deg_1800 - deg_900 * 2.0).abs() < 1e-9);
    }

    #[test]
    fn torque_zero_is_zero() {
        assert_eq!(normalize_torque(0), 0.0);
    }

    #[test]
    fn torque_max_is_100() {
        assert_eq!(normalize_torque(10_000), 100.0);
    }

    #[test]
    fn torque_min_is_neg_100() {
        assert_eq!(normalize_torque(-10_000), -100.0);
    }

    #[test]
    fn torque_half_is_50() {
        assert!((normalize_torque(5_000) - 50.0).abs() < 1e-9);
    }
}
