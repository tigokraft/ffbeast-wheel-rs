//! Sequential reader for packed C-style structs from a byte slice.

use crate::error::WheelError;

/// Sequential reader for packed C-style structs from a byte slice.
///
/// All multi-byte values are read as little-endian, matching the device's
/// `__attribute__((packed))` structs.
pub struct StructReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> StructReader<'a> {
    /// Creates a new reader positioned at the start of `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    /// Reads the next byte as `u8` and advances the cursor.
    pub fn u8(&mut self) -> Result<u8, WheelError> {
        self.require(1)?;
        let val = self.data[self.offset];
        self.offset += 1;
        Ok(val)
    }

    /// Reads the next byte as `i8` (reinterpret cast) and advances the cursor.
    pub fn i8(&mut self) -> Result<i8, WheelError> {
        Ok(self.u8()? as i8)
    }

    /// Reads the next 2 bytes as a little-endian `u16` and advances the cursor.
    pub fn u16(&mut self) -> Result<u16, WheelError> {
        self.require(2)?;
        let val = u16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        Ok(val)
    }

    /// Reads the next 2 bytes as a little-endian `i16` and advances the cursor.
    pub fn i16(&mut self) -> Result<i16, WheelError> {
        self.require(2)?;
        let val = i16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        Ok(val)
    }

    /// Reads the next 4 bytes as a little-endian `u32` and advances the cursor.
    pub fn u32(&mut self) -> Result<u32, WheelError> {
        self.require(4)?;
        let val = u32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        Ok(val)
    }

    fn require(&self, n: usize) -> Result<(), WheelError> {
        if self.offset + n > self.data.len() {
            Err(WheelError::BufferTooSmall {
                expected: self.offset + n,
                got: self.data.len(),
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_reader_u8() {
        let mut r = StructReader::new(&[0xAB]);
        assert_eq!(r.u8().unwrap(), 0xAB);
    }

    #[test]
    fn struct_reader_i8_signed() {
        let mut r = StructReader::new(&[0xFF]); // -1 as i8
        assert_eq!(r.i8().unwrap(), -1_i8);
    }

    #[test]
    fn struct_reader_u16_little_endian() {
        let mut r = StructReader::new(&[0x34, 0x12]); // 0x1234 in LE
        assert_eq!(r.u16().unwrap(), 0x1234);
    }

    #[test]
    fn struct_reader_i16_little_endian() {
        let mut r = StructReader::new(&[0xFF, 0xFF]); // -1 as i16 LE
        assert_eq!(r.i16().unwrap(), -1_i16);
    }

    #[test]
    fn struct_reader_u32_little_endian() {
        let mut r = StructReader::new(&[0x78, 0x56, 0x34, 0x12]); // 0x12345678 LE
        assert_eq!(r.u32().unwrap(), 0x1234_5678);
    }

    #[test]
    fn struct_reader_advances_offset() {
        let mut r = StructReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(r.u8().unwrap(), 0x01);
        assert_eq!(r.u8().unwrap(), 0x02);
        assert_eq!(r.u8().unwrap(), 0x03);
    }

    #[test]
    fn struct_reader_sequential_mixed_types() {
        // 1 u8 + 1 u16 LE = [0x01, 0x34, 0x12]
        let mut r = StructReader::new(&[0x01, 0x34, 0x12]);
        assert_eq!(r.u8().unwrap(), 0x01);
        assert_eq!(r.u16().unwrap(), 0x1234);
    }

    #[test]
    fn struct_reader_bounds_check_u8_empty() {
        let mut r = StructReader::new(&[]);
        assert!(matches!(
            r.u8(),
            Err(crate::error::WheelError::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn struct_reader_bounds_check_u16_one_byte() {
        let mut r = StructReader::new(&[0x01]); // only 1 byte available
        assert!(matches!(
            r.u16(),
            Err(crate::error::WheelError::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn struct_reader_bounds_check_after_partial_read() {
        let mut r = StructReader::new(&[0x01, 0x02]);
        r.u8().unwrap(); // consume 1 byte
        assert!(r.u16().is_err()); // only 1 left, need 2
    }
}
