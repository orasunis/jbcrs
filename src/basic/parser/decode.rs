//! The `decode` crate is used for decoding simple data,
//! like integers, floats and Strings.

use result::*;
use byteorder::{BigEndian, ByteOrder};
use std::char;

pub struct Decoder<'a> {
    bytes: &'a [u8],
    cursor: &'a mut usize,
    limit: usize,
}

impl<'a> Decoder<'a> {
    /// Creates a new decoder,
    /// the cursor has to be a mutable pointer to support limits without copying
    pub fn new(bytes: &'a [u8], cursor: &'a mut usize) -> Decoder<'a> {
        Decoder {
            bytes,
            cursor,
            limit: bytes.len(),
        }
    }

    /// Limits the decoder to `to` after the cursor
    pub fn limit(&mut self, to: usize) -> Result<Decoder> {
        let end = *self.cursor + to;
        self.check(end)?;
        Ok(Decoder {
            bytes: self.bytes,
            cursor: self.cursor,
            limit: end,
        })
    }

    /// Removes the limit and returns an error if the limit was exceeded, or not reached
    pub fn remove_limit(self) -> Result<()> {
        if self.limit == *self.cursor {
            Ok(())
        } else {
            Err(Error::LimitExceeded)
        }
    }

    /// Skips a certain amount of bytes and returns an error if it exceeded the limit
    pub fn skip(&mut self, to: usize) -> Result<()> {
        let end = *self.cursor + to;
        self.check(end)?;
        *self.cursor = end;
        Ok(())
    }

    /// Returns the current cursor
    pub fn cursor(&self) -> usize {
        *self.cursor
    }

    /// Reads a specific amount of bytes.
    /// If not enough bytes are available, an EOF error is returned.
    pub fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]> {
        let end = *self.cursor + count;
        self.check(end)?;

        let bytes = &self.bytes[*self.cursor..end];
        *self.cursor = end;
        Ok(bytes)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(1)?[0])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(BigEndian::read_u16(self.read_bytes(2)?))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(BigEndian::read_u32(self.read_bytes(4)?))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(BigEndian::read_u64(self.read_bytes(8)?))
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(BigEndian::read_f32(self.read_bytes(4)?))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(BigEndian::read_f64(self.read_bytes(8)?))
    }

    /// Decodes a modified UTF-8 string.
    /// Length is the amount of bytes the String was encoded in.
    /// The length used here may differ from the count of all chars.
    pub fn read_str(&mut self, length: usize) -> Result<String> {
        let mut out = String::with_capacity(length);

        let mut i = length;
        while i > 0 {
            // read first byte
            let r1 = u32::from(self.read_u8()?);
            let ch = if r1 != 0 && r1 < 0x80 {
                // single byte
                i -= 1;
                r1
            } else if r1 >= 0xC0 && r1 < 0xE0 && i >= 1 {
                // 2 bytes
                i -= 2;
                let r2 = u32::from(self.read_u8()?);
                (r1 & 0x1F) << 6 | (r2 & 0x3F)
            } else if r1 >= 0xE0 && r1 < 0xF0 && i >= 3 {
                i -= 3;
                let r2 = u32::from(self.read_u8()?);
                let r3 = u32::from(self.read_u8()?);
                if r1 == 0xED && r2 >= 0xA0 && r2 <= 0xAF {
                    if i >= 6 {
                        i -= 6;

                        self.read_u8()?;
                        let r5 = u32::from(self.read_u8()?);
                        let r6 = u32::from(self.read_u8()?);
                        // r1 and r4 can be ignored
                        0x1_0000 + ((r2 & 0x0F) << 16) + ((r3 & 0x3F) << 10) + ((r5 & 0x0F) << 6)
                            + (r6 & 0x3F)
                    } else {
                        return Err(Error::InvalidUTF8);
                    }
                } else {
                    ((r1 & 0x0F) << 12) + ((r2 & 0x3F) << 6) + (r3 & 0x3F)
                }
            } else {
                // this is not a valid utf8 scalar value
                return Err(Error::InvalidUTF8);
            };

            // convert the u32 to a char and push it to the output string
            let ch = char::from_u32(ch).ok_or(Error::InvalidUTF8)?;
            out.push(ch);
        }

        Ok(out)
    }

    /// Checks for bounds
    fn check(&self, location: usize) -> Result<()> {
        if location <= self.limit {
            Ok(())
        } else {
            Err(Error::LimitExceeded)
        }
    }
}

/// **Very** important tests (yes)
#[cfg(test)]
mod test {
    macro_rules! test_values {
        ( $func:ident { $( $input:expr => $expected:expr ),* } ) => {
            #[test]
            fn $func() {
                $(
                    let mut decoder = $crate::decode::Decoder::new(&$input);
                    for e in &$expected {
                        assert_eq!(decoder.$func().unwrap(), *e);
                    }
                )*
            }
        };
        ( $func:ident { $( $input:expr => $expected:expr,) + } ) => {
            test_values!{$func { $($input => $expected),+ }}
        };
    }

    test_values!{read_u8 {
        [0] => [0],
        [5] => [5],
        [0xFF] => [0xFF],
        [0xAA, 0xBB, 0xCC] => [0xAA, 0xBB, 0xCC],
    }}

    test_values!{read_u16 {
        [0x00, 0xFE] => [0x00FE],
        [0xAA, 0xBB] => [0xAABB],
        [0xFF, 0xFF, 0xCA, 0xFE] => [0xFFFF, 0xCAFE],
    }}

    test_values!{read_u32 {
        [0x00, 0xFE, 0x00, 0xAB] => [0x00FE00AB],
        [0xCA, 0xFE, 0xBA, 0xBE] => [0xCAFEBABE],
        [0x00, 0x00, 0x00, 0x00] => [0x00000000],
        [0xFF, 0xFF, 0xFF, 0xFF] => [0xFFFFFFFF],
    }}

    test_values!{read_u64 {
        [0x00, 0xFE, 0x00, 0xAB, 0xCD, 0x00, 0xEF, 0x00] => [0x00FE00AB_CD00EF00],
        [0xCA, 0xFE, 0xBA, 0xBE, 0xCA, 0xFE, 0xD0, 0x0D] => [0xCAFEBABE_CAFED00D],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] => [0x00000000_00000000],
        [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] => [0xFFFFFFFF_FFFFFFFF],
    }}

    test_values!{read_i8 {
        [0] => [0],
        [5] => [5],
        [0xFF] => [-1],
        [0x80] => [-128],
    }}

    test_values!{read_i16 {
        [0x00, 0xFE] => [0x00FE],
        [0xF5, 0x45] => [-0x0ABB],
        [0x80, 0xFF, 0xCA, 0xFE] => [-0x7F01, -0x3502],
    }}

    test_values!{read_i32 {
        [0xFF, 0x01, 0xFF, 0x55] => [-0x00FE00AB],
        [0xCA, 0xFE, 0xBA, 0xBE] => [-0x35014542],
        [0xFF, 0xFF, 0xFF, 0xFF] => [-0x00000001],
        [0x00, 0x00, 0x00, 0x00] => [ 0x00000000],
    }}

    test_values!{read_i64 {
        [0xA0, 0xFE, 0x00, 0xAB, 0xCD, 0x00, 0xEF, 0x00] => [-0x5F01FF54_32FF1100],
        [0xA5, 0x01, 0x45, 0x41, 0x35, 0x01, 0x2F, 0xF3] => [-0x5AFEBABE_CAFED00D],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] => [0x00000000_00000000],
        [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] => [0x7FFFFFFF_FFFFFFFF],
    }}

    #[test]
    fn read_str() {
        use super::Decoder;

        let mut decoder = Decoder::new("Hello, world!".as_bytes());
        assert_eq!(decoder.read_str(13).unwrap(), "Hello, world!".to_owned());
        // not complete, add more later.
    }

}
