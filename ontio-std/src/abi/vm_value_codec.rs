use super::Error;
use crate::abi::event_builder::TYPE_LIST;
use crate::abi::{VmValueBuilder, VmValueParser};
use crate::prelude::*;
pub trait VmValueEncoder {
    fn serialize(&self, sink: &mut VmValueBuilder);
}

impl VmValueEncoder for &str {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.string(self);
    }
}

impl VmValueEncoder for &[u8] {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.bytearray(self);
    }
}

impl VmValueEncoder for bool {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.bool(*self);
    }
}

impl VmValueEncoder for H256 {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.h256(&self);
    }
}

impl VmValueEncoder for U128 {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.number(self.clone());
    }
}

impl VmValueEncoder for Address {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.address(&self);
    }
}

impl<T: VmValueEncoder> VmValueEncoder for &T {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        (*self).serialize(builder)
    }
}

pub trait VmValueDecoder<'a>: Sized {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error>;
}

impl<'a> VmValueDecoder<'a> for &'a str {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.string()
    }
}

impl<'a> VmValueDecoder<'a> for &'a [u8] {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.bytearray()
    }
}

impl<'a> VmValueDecoder<'a> for bool {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.bool()
    }
}

impl<'a> VmValueDecoder<'a> for &'a H256 {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.h256()
    }
}

impl<'a> VmValueDecoder<'a> for Vec<u8> {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.bytearray_vec()
    }
}

impl<'a, T: VmValueDecoder<'a>> VmValueDecoder<'a> for Vec<T> {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        let ty = parser.source.read_byte()?;
        if ty != TYPE_LIST {
            return Err(Error::TypeInconsistency);
        }
        let len = parser.source.read_u32()?;
        let mut value = Vec::with_capacity(cmp::min(len, 1024) as usize);
        for _i in 0..len {
            value.push(parser.read::<T>()?);
        }
        Ok(value)
    }
}

impl<'a> VmValueDecoder<'a> for H256 {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        let r = parser.h256()?;
        Ok(r.clone())
    }
}

impl<'a> VmValueDecoder<'a> for U128 {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.number()
    }
}

impl<'a> VmValueDecoder<'a> for &'a Address {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.address()
    }
}
