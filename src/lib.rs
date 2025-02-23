pub mod array_property;
pub mod bool_property;
mod common;
pub mod float_property;
pub mod int_property;
pub mod map_property;
pub mod set_property;
pub mod str_property;
pub mod struct_property;
mod structs;

use binrw::helpers::{until, until_eof};

use crate::array_property::ArrayProperty;
use crate::bool_property::BoolProperty;
use crate::float_property::FloatProperty;
use crate::int_property::IntProperty;
use crate::map_property::MapProperty;
use crate::set_property::SetProperty;
use crate::str_property::StrProperty;
use crate::struct_property::StructProperty;
use binrw::binrw;
use crate::common::read_string_with_length;

#[binrw]
#[derive(Debug)]
pub enum Property {
    #[br(magic = b"IntProperty\0")]
    Int(IntProperty),
    #[br(magic = b"BoolProperty\0")]
    Bool(BoolProperty),
    #[br(magic = b"StructProperty\0")]
    Struct(StructProperty),
    #[br(magic = b"FloatProperty\0")]
    Float(FloatProperty),
    #[br(magic = b"StrProperty\0")]
    String(StrProperty),
    #[br(magic = b"NameProperty\0")]
    Name(StrProperty),
    #[br(magic = b"ArrayProperty\0")]
    Array(ArrayProperty),
    #[br(magic = b"MapProperty\0")]
    Map(MapProperty),
    #[br(magic = b"SetProperty\0")]
    Set(SetProperty),
}

// Used in ArrayProperty exclusively, but could be used instead of magic above
#[binrw]
#[derive(Debug)]
#[br(import { magic: &str })]
pub enum StringBasedProperty {
    #[br(pre_assert("NameProperty" == magic))]
    Name(StrProperty),
    #[br(pre_assert("StructProperty" == magic))]
    Struct(StructProperty),
    #[br(pre_assert("FloatProperty" == magic))]
    Float(FloatProperty),
    #[br(pre_assert("StrProperty" == magic))]
    String(StrProperty),
    #[br(pre_assert("BoolProperty" == magic))]
    Bool(BoolProperty),
    #[br(pre_assert("IntProperty" == magic))]
    Int(IntProperty),
    #[br(pre_assert("ArrayProperty" == magic))]
    Array(ArrayProperty),
    #[br(pre_assert("MapProperty" == magic))]
    Map(MapProperty),
    #[br(pre_assert("SetProperty" == magic))]
    Set(SetProperty),
}

#[binrw]
#[derive(Debug)]
pub struct Entry {
    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    #[br(temp, if(name != "None"))]
    pub type_name: String,

    #[br(if(name != "None"), args { magic: &type_name })]
    pub r#type: Option<StringBasedProperty>,
}

#[binrw]
#[derive(Debug)]
pub struct TaggedObject {
    pub size_in_bytes: u32,
    #[br(parse_with = until(|entry: &Entry| entry.name == "None"))]
    pub entries: Vec<Entry>,
}

#[binrw]
#[derive(Debug)]
pub struct TaggedSerialization {
    // Excluding the first four bytes, which is this
    pub size_in_bytes: u32,
    #[br(parse_with = until_eof)]
    pub objs: Vec<TaggedObject>,
}

#[binrw]
#[derive(Debug)]
pub struct CompressedSaveFile {
    pub magic: u64,
    pub compressed_size: u64,
    pub a_compresed_size: u64,
    pub a_uncompresed_size: u64,
    pub b_compresed_size: u64,
    pub b_uncompresed_size: u64,
    #[br(count = a_compresed_size)]
    pub compressed_data: Vec<u8>,
}
