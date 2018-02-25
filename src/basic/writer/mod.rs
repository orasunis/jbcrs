mod encode;

use result::*;
use super::constpool::*;
use super::tree::*;
use self::encode::Encoder;

/// Writes a constant pool and class to a byte vector
pub fn write(constant_pool: &Pool, class: &Class) -> Result<Vec<u8>> {
    let mut encoder = Encoder::new();

    encoder.write_bytes(MAGIC);
    encoder.write_u16(class.minor_version);
    encoder.write_u16(class.major_version);

    Ok(encoder.bytes())
}
