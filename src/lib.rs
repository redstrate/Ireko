pub mod array_property;
pub mod bool_property;
mod build_data;
mod common;
mod enum_property;
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

use binrw::helpers::until_eof;
use std::fs::write;
use std::io::{Cursor, Read};

use crate::array_property::ArrayProperty;
use crate::bool_property::BoolProperty;
use crate::common::{read_string_with_length, write_string_with_length};
use crate::float_property::FloatProperty;
use crate::int_property::IntProperty;
use crate::map_property::MapProperty;
use crate::name_property::NameProperty;
use crate::set_property::SetProperty;
use crate::str_property::StrProperty;
use crate::struct_property::StructProperty;
use binrw::{BinRead, BinResult, BinWrite, binrw};
use flate2::bufread::ZlibDecoder;

// Used in ArrayProperty exclusively, but could be used instead of magic above
#[binrw]
#[derive(Debug)]
#[br(import { magic: &str, name: &str })]
pub enum Property {
    #[br(pre_assert("NameProperty" == magic))]
    Name(NameProperty),
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
    let end = current + size_in_bytes as u64 - 4; // for the size bits

    while current < end {
        let entry = Entry::read_options(reader, endian, ())?;
        if entry.name == "None" {
            break;
        }
        result.push(entry);
        current = reader.stream_position()?;
    }
    Ok(result)
}

#[binrw::writer(writer, endian)]
fn custom_tagged_object_writer(entries: &Vec<Entry>) -> BinResult<()> {
    for entry in entries {
        entry.write_options(writer, endian, ())?
    }
    // Write "none" entry at the end
    let none_entry = Entry {
        name: "None".to_string(),
        type_name: "".to_string(),
        r#type: None,
    };
    none_entry.write_options(writer, endian, ())?;
    Ok(())
}

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32) -> BinResult<Vec<TaggedObject>> {
    let mut result = Vec::<TaggedObject>::new();

    let mut current = reader.stream_position()?;
    let end = current + size_in_bytes as u64;

    while current < end {
        let obj = TaggedObject::read_options(reader, endian, ())?;
        if obj.size_in_bytes == 0 {
            break;
        }
        result.push(obj);
        current = reader.stream_position()?;
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
pub struct TaggedObject {
    pub size_in_bytes: u32,
    #[br(parse_with = custom_tagged_object_parser, args(size_in_bytes))]
    #[bw(write_with = custom_tagged_object_writer)]
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
    pub size_in_bytes: u32,
    #[br(parse_with = custom_parser, args(size_in_bytes))]
    pub objs: Vec<TaggedObject>,
}

#[binrw::parser(reader, endian)]
fn read_compressed_data(compressed_size: u64, uncompressed_size: u64) -> BinResult<Vec<u8>> {
    let mut compressed_data = vec![0; compressed_size as usize];
    reader.read_exact(&mut compressed_data)?;

    let mut uncompressed = vec![0; uncompressed_size as usize];

    let mut d = ZlibDecoder::new(&*compressed_data);
    let size = d.read(&mut uncompressed)?;

    Ok(uncompressed[..size].to_vec())
}

// TODO: there's no point in using a parser, we should just use map()
#[binrw::parser(reader, endian)]
fn read_tagged_data(data_blocks: &Vec<CompressedBlock>) -> BinResult<TaggedSerialization> {
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
    #[br(temp)]
    #[bw(ignore)]
    pub data: Vec<CompressedBlock>,
    #[br(parse_with = read_tagged_data, args(&data))]
    pub value: TaggedSerialization,
}
