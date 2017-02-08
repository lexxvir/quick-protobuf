//! Automatically generated rust module for 'data_types.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::io::{Write};
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

use super::data_types_import::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FooEnum {
    FIRST_VALUE = 1,
    SECOND_VALUE = 2,
}

impl Default for FooEnum {
    fn default() -> Self {
        FooEnum::FIRST_VALUE
    }
}

impl From<i32> for FooEnum {
    fn from(i: i32) -> Self {
        match i {
            1 => FooEnum::FIRST_VALUE,
            2 => FooEnum::SECOND_VALUE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BarMessage {
    pub b_required_int32: i32,
}

impl BarMessage {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.b_required_int32 = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BarMessage {
    fn get_size(&self) -> usize {
        sizeof_varint(*&self.b_required_int32 as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_varint(*&self.b_required_int32 as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FooMessage<'a> {
    pub f_int32: Option<i32>,
    pub f_int64: Option<i64>,
    pub f_uint32: Option<u32>,
    pub f_uint64: Option<u64>,
    pub f_sint32: Option<i32>,
    pub f_sint64: Option<i64>,
    pub f_bool: Option<bool>,
    pub f_FooEnum: Option<FooEnum>,
    pub f_fixed64: Option<u64>,
    pub f_sfixed64: Option<i64>,
    pub f_fixed32: Option<u32>,
    pub f_sfixed32: Option<i32>,
    pub f_double: Option<f64>,
    pub f_float: Option<f32>,
    pub f_bytes: Option<Cow<'a, [u8]>>,
    pub f_string: Option<Cow<'a, str>>,
    pub f_self_message: Option<Box<FooMessage<'a>>>,
    pub f_bar_message: Option<BarMessage>,
    pub f_repeated_int32: Vec<i32>,
    pub f_repeated_packed_int32: Vec<i32>,
    pub f_imported: Option<mod_a::mod_b::ImportedMessage>,
    pub f_baz: Option<BazMessage>,
    pub f_nested: Option<mod_BazMessage::Nested>,
}

impl<'a> FooMessage<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.f_int32 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.f_int64 = Some(r.read_int64(bytes)?),
                Ok(24) => msg.f_uint32 = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.f_uint64 = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.f_sint32 = Some(r.read_sint32(bytes)?),
                Ok(48) => msg.f_sint64 = Some(r.read_sint64(bytes)?),
                Ok(56) => msg.f_bool = Some(r.read_bool(bytes)?),
                Ok(64) => msg.f_FooEnum = Some(r.read_enum(bytes)?),
                Ok(73) => msg.f_fixed64 = Some(r.read_fixed64(bytes)?),
                Ok(81) => msg.f_sfixed64 = Some(r.read_sfixed64(bytes)?),
                Ok(93) => msg.f_fixed32 = Some(r.read_fixed32(bytes)?),
                Ok(101) => msg.f_sfixed32 = Some(r.read_sfixed32(bytes)?),
                Ok(105) => msg.f_double = Some(r.read_double(bytes)?),
                Ok(117) => msg.f_float = Some(r.read_float(bytes)?),
                Ok(122) => msg.f_bytes = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.f_string = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.f_self_message = Some(Box::new(r.read_message(bytes, FooMessage::from_reader)?)),
                Ok(146) => msg.f_bar_message = Some(r.read_message(bytes, BarMessage::from_reader)?),
                Ok(152) => msg.f_repeated_int32.push(r.read_int32(bytes)?),
                Ok(162) => msg.f_repeated_packed_int32 = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(170) => msg.f_imported = Some(r.read_message(bytes, mod_a::mod_b::ImportedMessage::from_reader)?),
                Ok(178) => msg.f_baz = Some(r.read_message(bytes, BazMessage::from_reader)?),
                Ok(186) => msg.f_nested = Some(r.read_message(bytes, mod_BazMessage::Nested::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FooMessage<'a> {
    fn get_size(&self) -> usize {
        self.f_int32.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_int64.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_uint32.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_uint64.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_sint32.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_sint64.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_bool.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_FooEnum.as_ref().map_or(0, |m| sizeof_varint(*m as u64))
        + self.f_fixed64.as_ref().map_or(0, |_| 8)
        + self.f_sfixed64.as_ref().map_or(0, |_| 8)
        + self.f_fixed32.as_ref().map_or(0, |_| 4)
        + self.f_sfixed32.as_ref().map_or(0, |_| 4)
        + self.f_double.as_ref().map_or(0, |_| 8)
        + self.f_float.as_ref().map_or(0, |_| 4)
        + self.f_bytes.as_ref().map_or(0, |m| m.len())
        + self.f_string.as_ref().map_or(0, |m| m.len())
        + self.f_self_message.as_ref().map_or(0, |m| m.get_size())
        + self.f_bar_message.as_ref().map_or(0, |m| m.get_size())
        + self.f_repeated_int32.iter().map(|s| 2 + sizeof_varint(*s as u64)).sum::<usize>()
        + if self.f_repeated_packed_int32.is_empty() { 0 } else { 2 + sizeof_var_length(self.f_repeated_packed_int32.iter().map(|s| sizeof_varint(*s as u64)).sum::<usize>()) }
        + self.f_imported.as_ref().map_or(0, |m| m.get_size())
        + self.f_baz.as_ref().map_or(0, |m| m.get_size())
        + self.f_nested.as_ref().map_or(0, |m| m.get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.f_int32 { w.write_with_tag(8, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_int64 { w.write_with_tag(16, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_uint32 { w.write_with_tag(24, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_uint64 { w.write_with_tag(32, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_sint32 { w.write_with_tag(40, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_sint64 { w.write_with_tag(48, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_bool { w.write_with_tag(56, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_FooEnum { w.write_with_tag(64, |w| w.write_varint(*s as u64))?; }
        if let Some(ref s) = self.f_fixed64 { w.write_with_tag(73, |w| w.write_fixed64(*s))?; }
        if let Some(ref s) = self.f_sfixed64 { w.write_with_tag(81, |w| w.write_sfixed64(*s))?; }
        if let Some(ref s) = self.f_fixed32 { w.write_with_tag(93, |w| w.write_fixed32(*s))?; }
        if let Some(ref s) = self.f_sfixed32 { w.write_with_tag(101, |w| w.write_sfixed32(*s))?; }
        if let Some(ref s) = self.f_double { w.write_with_tag(105, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.f_float { w.write_with_tag(117, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.f_bytes { w.write_with_tag(122, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.f_string { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.f_self_message { w.write_with_tag(138, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.f_bar_message { w.write_with_tag(146, |w| w.write_message(s))?; }
        for s in &self.f_repeated_int32 { w.write_with_tag(152, |w| w.write_varint(*s as u64))?; }
        w.write_packed_with_tag(162, &self.f_repeated_packed_int32, |w, m| w.write_varint(*m as u64), &|m| sizeof_varint(*m as u64))?;
        if let Some(ref s) = self.f_imported { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.f_baz { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.f_nested { w.write_with_tag(186, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BazMessage {
    pub nested: Option<mod_BazMessage::Nested>,
}

impl BazMessage {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nested = Some(r.read_message(bytes, mod_BazMessage::Nested::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BazMessage {
    fn get_size(&self) -> usize {
        self.nested.as_ref().map_or(0, |m| m.get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.nested { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_BazMessage {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Nested {
    pub f_nested: i32,
}

impl Nested {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.f_nested = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Nested {
    fn get_size(&self) -> usize {
        sizeof_varint(*&self.f_nested as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_varint(*&self.f_nested as u64))?;
        Ok(())
    }
}

}
