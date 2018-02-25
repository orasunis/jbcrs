use byteorder::{BigEndian, ByteOrder};

/// Encodes primitive types to a byte vector
pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder { buf: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Encoder {
        Encoder {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn bytes(self) -> Vec<u8> {
        self.buf
    }

    /// Writes a byte array to the buffer
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    /// Writes a u8 to the buffer
    pub fn write_u8(&mut self, u: u8) {
        self.buf.push(u);
    }

    /// Writes a u16 to the buffer
    pub fn write_u16(&mut self, u: u16) {
        let mut buf = [0; 2];
        BigEndian::write_u16(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a u32 to the buffer
    pub fn write_u32(&mut self, u: u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a u64 to the buffer
    pub fn write_u64(&mut self, u: u64) {
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a i8 to the buffer
    pub fn write_i8(&mut self, u: i8) {
        self.buf.push(u as u8);
    }

    /// Writes a i16 to the buffer
    pub fn write_i16(&mut self, u: i16) {
        let mut buf = [0; 2];
        BigEndian::write_i16(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a i32 to the buffer
    pub fn write_i32(&mut self, u: i32) {
        let mut buf = [0; 4];
        BigEndian::write_i32(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a i64 to the buffer
    pub fn write_i64(&mut self, u: i64) {
        let mut buf = [0; 8];
        BigEndian::write_i64(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a f32 to the buffer
    pub fn write_f32(&mut self, u: f32) {
        let mut buf = [0; 4];
        BigEndian::write_f32(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a f64 to the buffer
    pub fn write_f64(&mut self, u: f64) {
        let mut buf = [0; 8];
        BigEndian::write_f64(&mut buf, u);
        self.write_bytes(&buf);
    }

    /// Writes a modified UTF-8 string to the buffer
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            let u = c as u32;
            if u != 0x00 && u < 0x80 {
                self.buf.push(u as u8);
            } else if u < 0x0800 {
                self.buf
                    .extend_from_slice(&[0xC0 | ((u >> 6) & 0x1F) as u8, 0x80 | (u & 0x3F) as u8]);
            } else if u < 0x1_0000 {
                self.buf.extend_from_slice(&[
                    0xE0 | ((u >> 12) & 0x0F) as u8,
                    0x80 | ((u >> 6) & 0x3F) as u8,
                    0x80 | (u & 0x3F) as u8,
                ]);
            } else {
                self.buf.extend_from_slice(&[
                    0xED,
                    0xA0 | ((u >> 16) & 0x0F) as u8,
                    0x80 | ((u >> 12) & 0x3F) as u8,
                    0xED,
                    0xB0 | ((u >> 6) & 0x0F) as u8,
                    0x80 | (u & 0x3F) as u8,
                ]);
            }
        }
    }
}
