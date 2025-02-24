pub mod array_property;
pub mod bool_property;
mod build_data;
mod common;
pub mod float_property;
mod guid;
pub mod int_property;
mod linear_color;
pub mod map_property;
mod name_property;
mod primary_asset_id;
mod primary_asset_type;
pub mod set_property;
pub mod str_property;
pub mod struct_property;
mod structs;

use std::fs::write;
use std::io::{Cursor, Read};
use binrw::helpers::until_eof;

use crate::array_property::{ArrayEntry, ArrayKeyData, ArrayProperty};
use crate::bool_property::BoolProperty;
use crate::common::{read_string_with_length, write_string_with_length};
use crate::float_property::FloatProperty;
use crate::int_property::IntProperty;
use crate::map_property::{KeyType, MapProperty};
use crate::set_property::SetProperty;
use crate::str_property::StrProperty;
use crate::struct_property::StructProperty;
use binrw::{BinRead, BinResult, binrw};
use flate2::bufread::ZlibDecoder;
use flate2::Decompress;

// Used in ArrayProperty exclusively, but could be used instead of magic above
#[binrw]
#[derive(Debug)]
#[br(import { magic: &str, name: &str })]
pub enum Property {
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
    Set {
        #[br(args(name))]
        value: SetProperty,
    },
}

#[binrw]
#[derive(Debug)]
pub struct Entry {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(if(name != "None"))]
    pub type_name: String,

    #[br(if(name != "None"), args { magic: &type_name, name: &name })]
    pub r#type: Option<Property>,
}

#[binrw::parser(reader, endian)]
fn custom_tagged_object_parser(size_in_bytes: u32) -> BinResult<Vec<Entry>> {
    let mut result = Vec::<Entry>::new();

    let mut current = reader.stream_position()?;
    let end = current + size_in_bytes as u64;

    while current < end {
        result.push(Entry::read_options(reader, endian, ())?);
        current = reader.stream_position()?;
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
pub struct TaggedObject {
    pub size_in_bytes: u32,
    //#[br(parse_with = custom_tagged_object_parser, args(size_in_bytes))]
    //#[br(parse_with = until(|entry: &Entry| entry.name == "None"))]
    #[br(parse_with = until_eof)]
    pub entries: Vec<Entry>,
}

impl TaggedObject {
    pub fn entry(&self, key: &str) -> Option<&Entry> {
        let entries: Vec<&Entry> = self.entries.iter().filter(|e| e.name == key).collect();

        entries.first().copied()
    }
}

#[binrw]
#[derive(Debug)]
pub struct TaggedSerialization {
    // Excluding the first four bytes, which is this
    pub size_in_bytes: u32,
    #[br(parse_with = until_eof)]
    pub objs: Vec<TaggedObject>,
}

#[binrw::parser(reader, endian)]
fn read_compressed_data(
    compressed_size: u64,
    uncompressed_size: u64,
) -> BinResult<Vec<u8>> {
    let mut compressed_data = vec![0; compressed_size as usize];
    reader.read_exact(&mut compressed_data)?;

    let mut uncompressed = vec![0; uncompressed_size as usize];

    let mut d = ZlibDecoder::new(&*compressed_data);
    let size = d.read(&mut uncompressed)?;

    Ok(uncompressed[..size].to_vec())
}

// TODO: there's no point in using a parser, we should just use map()
#[binrw::parser(reader, endian)]
fn read_tagged_data(
    data_blocks: &Vec<CompressedBlock>
) -> BinResult<TaggedSerialization> {
    let data_vecs: Vec<Vec<u8>> = data_blocks.iter().map(|x| x.data.clone()).collect();
    let combined_data = data_vecs.concat();
    write("output.bin", &combined_data);

    let mut cursor = Cursor::new(&combined_data);

    TaggedSerialization::read_le(&mut cursor)
}

#[binrw]
#[brw(magic = 0x9e2a83c1u32)]
#[derive(Debug)]
pub struct CompressedBlock {
    #[br(pad_before = 6)]
    #[br(pad_after = 5)]
    pub unk: u8, // always 2
    pub a_compresed_size: u64,
    pub a_uncompresed_size: u64,
    pub b_compresed_size: u64,
    pub b_uncompresed_size: u64,
    #[br(parse_with = read_compressed_data, args(a_compresed_size, a_uncompresed_size))]
    pub data: Vec<u8>,
}

#[binrw]
#[derive(Debug)]
pub struct CompressedSaveFile {
    #[br(parse_with = until_eof)]
    pub data: Vec<CompressedBlock>,
    #[br(parse_with = read_tagged_data, args(&data))]
    pub value: TaggedSerialization,
}
