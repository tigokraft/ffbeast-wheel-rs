//! License key parsing.

use crate::error::WheelError;

/// Parses a firmware activation license key string into three `u32` chunks.
///
/// Expected format: `"XXXXXXXX-XXXXXXXX-XXXXXXXX"` (three 8-character
/// uppercase or lowercase hex segments separated by hyphens).
///
/// Returns `Err(`[`WheelError::InvalidLicense`]`)` on any format violation.
pub(crate) fn parse_license_key(license: &str) -> Result<[u32; 3], WheelError> {
    let chunks: Vec<&str> = license.trim().split('-').collect();
    if chunks.len() != 3 {
        return Err(WheelError::InvalidLicense(
            "expected 3 hyphen-separated segments".into(),
        ));
    }
    let mut out = [0u32; 3];
    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.len() != 8 || !chunk.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(WheelError::InvalidLicense(format!(
                "segment {} must be exactly 8 hex characters (got {:?})",
                i + 1,
                chunk
            )));
        }
        out[i] = u32::from_str_radix(chunk, 16)
            .map_err(|_| WheelError::InvalidLicense(format!("cannot parse segment {}", i + 1)))?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_valid_parses_three_u32s() {
        let key = parse_license_key("AABBCCDD-11223344-DEADBEEF").unwrap();
        assert_eq!(key[0], 0xAABB_CCDD);
        assert_eq!(key[1], 0x1122_3344);
        assert_eq!(key[2], 0xDEAD_BEEF);
    }

    #[test]
    fn license_lowercase_hex_accepted() {
        let key = parse_license_key("aabbccdd-11223344-deadbeef").unwrap();
        assert_eq!(key[0], 0xAABB_CCDD);
    }

    #[test]
    fn license_with_leading_trailing_whitespace_accepted() {
        assert!(parse_license_key("  AABBCCDD-11223344-DEADBEEF  ").is_ok());
    }

    #[test]
    fn license_wrong_segment_count_is_err() {
        assert!(parse_license_key("AABBCCDD-11223344").is_err());
        assert!(parse_license_key("AABBCCDD-11223344-DEADBEEF-00000000").is_err());
    }

    #[test]
    fn license_short_segment_is_err() {
        assert!(parse_license_key("AABB-11223344-DEADBEEF").is_err());
    }

    #[test]
    fn license_non_hex_chars_are_err() {
        assert!(parse_license_key("GGHHIIJJ-11223344-DEADBEEF").is_err());
    }
}
