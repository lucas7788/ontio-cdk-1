use super::Encoder;
use crate::Vec;
use byteorder::{ByteOrder, LittleEndian};

pub struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    pub fn new(cap: usize) -> Self {
        Sink {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn write<T: Encoder>(&mut self, val: T) {
        val.encode(self)
    }

    pub(crate) fn write_byte(&mut self, b: u8) {
        self.buf.push(b)
    }

    pub(crate) fn write_bool(&mut self, b: bool) {
        if b {
            self.write_byte(1)
        } else {
            self.write_byte(0)
        }
    }
    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }

    pub(crate) fn write_u16(&mut self, val: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_u32(&mut self, val: u32) {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_u64(&mut self, val: u64) {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_varuint(&mut self, val: u64) {
        if val < 0xFD {
            self.write_byte(val as u8);
        } else if val < 0xFFFF {
            self.write_byte(0xFD);
            self.write_u16(val as u16);
        } else if val <= 0xFFFFFFFF {
            self.write_byte(0xFE);
            self.write_u32(val as u32);
        } else {
            self.write_byte(0xFF);
            self.write_u64(val);
        }
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}
