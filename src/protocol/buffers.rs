//! HID output buffer construction helpers.

use crate::constants::REPORT_SIZE;
use crate::enums::{field_type_for, FieldType, ReportData, ReportType};
use crate::types::DirectControl;
use crate::enums::SettingField;

/// Builds the 65-byte HID output buffer for a single setting-field write.
///
/// Buffer layout:
/// ```text
/// [0]    report ID        = 0xA3 (GenericInputOutput)
/// [1]    report data      = 0x14 (DataSettingsFieldData)
/// [2]    field identifier
/// [3]    field index
/// [4…]   value encoded as the field's native type (little-endian)
/// ```
pub(crate) fn build_setting_buf(field: SettingField, index: u8, value: i64) -> [u8; REPORT_SIZE + 1] {
    let mut buf = [0u8; REPORT_SIZE + 1];
    buf[0] = ReportType::GenericInputOutput as u8;
    buf[1] = ReportData::DataSettingsFieldData as u8;
    buf[2] = field as u8;
    buf[3] = index;

    const VAL: usize = 4;
    match field_type_for(field) {
        FieldType::Int8 => {
            buf[VAL] = value.clamp(-128, 127) as i8 as u8;
        }
        FieldType::Uint8 => {
            buf[VAL] = value.clamp(0, 255) as u8;
        }
        FieldType::Int16 => {
            buf[VAL..VAL + 2]
                .copy_from_slice(&(value.clamp(-32768, 32767) as i16).to_le_bytes());
        }
        FieldType::Uint16 => {
            buf[VAL..VAL + 2]
                .copy_from_slice(&(value.clamp(0, 65535) as u16).to_le_bytes());
        }
        FieldType::Float32 => {
            buf[VAL..VAL + 4].copy_from_slice(&(value as f32).to_le_bytes());
        }
        FieldType::Int32 => {
            buf[VAL..VAL + 4].copy_from_slice(
                &(value.clamp(i32::MIN as i64, i32::MAX as i64) as i32).to_le_bytes(),
            );
        }
        FieldType::Uint32 => {
            buf[VAL..VAL + 4]
                .copy_from_slice(&(value.clamp(0, u32::MAX as i64) as u32).to_le_bytes());
        }
    }
    buf
}

/// Builds the 65-byte HID output buffer for a direct-control write.
///
/// Buffer layout:
/// ```text
/// [0]    report ID      = 0xA3
/// [1]    report data    = 0x10 (DataOverrideData)
/// [2–3]  spring_force   (i16 LE, clamped to ±10 000)
/// [4–5]  constant_force (i16 LE, clamped to ±10 000)
/// [6–7]  periodic_force (i16 LE, clamped to ±10 000)
/// [8]    force_drop     (u8, clamped to 0–100)
/// ```
pub(crate) fn build_direct_control_buf(control: &DirectControl) -> [u8; REPORT_SIZE + 1] {
    let mut buf = [0u8; REPORT_SIZE + 1];
    buf[0] = ReportType::GenericInputOutput as u8;
    buf[1] = ReportData::DataOverrideData as u8;
    buf[2..4].copy_from_slice(&control.spring_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[4..6].copy_from_slice(&control.constant_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[6..8].copy_from_slice(&control.periodic_force.clamp(-10_000, 10_000).to_le_bytes());
    buf[8] = control.force_drop.min(100);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_buf_header_bytes() {
        let buf = build_setting_buf(SettingField::MotionRange, 0, 900);
        assert_eq!(buf[0], ReportType::GenericInputOutput as u8);
        assert_eq!(buf[1], ReportData::DataSettingsFieldData as u8);
        assert_eq!(buf[2], SettingField::MotionRange as u8);
        assert_eq!(buf[3], 0);
    }

    #[test]
    fn setting_buf_u16_value_little_endian() {
        let buf = build_setting_buf(SettingField::MotionRange, 0, 900);
        assert_eq!(u16::from_le_bytes([buf[4], buf[5]]), 900);
    }

    #[test]
    fn setting_buf_i8_negative_value() {
        let buf = build_setting_buf(SettingField::DirectXConstantDirection, 0, -1);
        assert_eq!(buf[4] as i8, -1_i8);
    }

    #[test]
    fn setting_buf_u8_clamps_negative_to_zero() {
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, -5);
        assert_eq!(buf[4], 0);
    }

    #[test]
    fn setting_buf_u8_clamps_overflow_to_255() {
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, 999);
        assert_eq!(buf[4], 255);
    }

    #[test]
    fn setting_buf_index_is_stored() {
        let buf = build_setting_buf(SettingField::PinMode, 7, 1);
        assert_eq!(buf[3], 7);
    }

    #[test]
    fn setting_buf_tail_is_zero_padded() {
        let buf = build_setting_buf(SettingField::TotalEffectStrength, 0, 42);
        assert!(buf[5..].iter().all(|&b| b == 0));
    }

    #[test]
    fn direct_control_buf_header() {
        let buf = build_direct_control_buf(&DirectControl::default());
        assert_eq!(buf[0], ReportType::GenericInputOutput as u8);
        assert_eq!(buf[1], ReportData::DataOverrideData as u8);
    }

    #[test]
    fn direct_control_buf_forces_little_endian() {
        let ctrl = DirectControl {
            spring_force: 1_000,
            constant_force: -500,
            periodic_force: 0,
            force_drop: 25,
        };
        let buf = build_direct_control_buf(&ctrl);
        assert_eq!(i16::from_le_bytes([buf[2], buf[3]]), 1_000);
        assert_eq!(i16::from_le_bytes([buf[4], buf[5]]), -500);
        assert_eq!(i16::from_le_bytes([buf[6], buf[7]]), 0);
        assert_eq!(buf[8], 25);
    }

    #[test]
    fn direct_control_buf_clamps_forces() {
        let ctrl = DirectControl {
            spring_force: 30_000,
            constant_force: -30_000,
            periodic_force: 0,
            force_drop: 200,
        };
        let buf = build_direct_control_buf(&ctrl);
        assert_eq!(i16::from_le_bytes([buf[2], buf[3]]), 10_000);
        assert_eq!(i16::from_le_bytes([buf[4], buf[5]]), -10_000);
        assert_eq!(buf[8], 100);
    }
}
