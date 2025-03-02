use binrw::{BinRead, BinResult, BinWrite, binrw};

use crate::{
    common::{read_string_with_length, write_string_with_length},
    property::{
        BoolProperty, FloatProperty, IntProperty, NameProperty, StrProperty, StructProperty,
        array_property::ArrayProperty, map_property::MapProperty, set_property::SetProperty,
    },
};

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
    Set(SetProperty),
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

#[binrw]
#[derive(Debug)]
pub struct GenericTaggedObject {
    pub size_in_bytes: u32,
    #[br(parse_with = custom_tagged_object_parser, args(size_in_bytes))]
    #[bw(write_with = custom_tagged_object_writer)]
    pub entries: Vec<Entry>,
}

impl GenericTaggedObject {
    pub fn entry(&self, key: &str) -> Option<&Entry> {
        let entries: Vec<&Entry> = self.entries.iter().filter(|e| e.name == key).collect();
        entries.first().copied()
    }
}
