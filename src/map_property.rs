use std::io::{Read, Seek, SeekFrom};
use crate::read_bool_from;
use crate::struct_property::Struct;
use crate::structs::PrimaryAssetNameProperty;
use binrw::{binrw, BinRead, BinResult};
use binrw::helpers::until_exclusive;

// A struct without a name
#[binrw]
#[derive(Debug)]
pub struct AnonymousStruct {
    #[br(parse_with = until_exclusive(|entry: &PrimaryAssetNameProperty| entry.property_name == "None"))]
    fields: Vec<PrimaryAssetNameProperty>,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubStructProperty {
    #[br(temp)]
    #[bw(ignore)]
    pub unk_name_length: u32,
    #[br(args { is_map: true })]
    pub r#struct: Struct,
}

#[binrw]
#[derive(Debug)]
pub struct StructMaybeKey {
    #[br(temp)]
    #[bw(ignore)]
    pub unk_name_length: u32,
    #[br(count = unk_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub unk_name: String,

    #[br(temp)]
    #[bw(ignore)]
    pub unk_type_length: u32,
    #[br(count = unk_type_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    #[br(pad_after = 8)] // nothingness and then length for the struct
    pub unk_type: String,

    #[br(dbg)]
    #[br(pad_before = 4)] // type length
    #[br(args { is_map: true })]
    pub r#struct: Struct,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubFloatProperty {
    pub value: f32,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubNameProperty {
    #[br(temp)]
    #[bw(ignore)]
    pub sub_property_name_length: u32,
    #[br(count = sub_property_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub sub_property_name: String,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubStrProperty {
    #[br(temp)]
    #[bw(ignore)]
    pub name_length: u32,
    #[br(count = name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub name: String,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubBoolProperty {
    #[br(map = read_bool_from::<u8>)]
    #[bw(ignore)]
    pub value: bool,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubIntProperty {
    pub value: u32,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubEnumProperty {
    pub enum_length: u32,
    #[br(count = enum_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub r#enum: String,
}

#[binrw]
#[derive(Debug)]
pub struct GuidStruct {
    pub guid: [u8; 16],
}

// Used in MapProperty exclusively, seems to be a shortened version of some Properties
#[binrw]
#[derive(Debug)]
#[br(import { magic: &str, is_value: bool })]
pub enum MabSubProperty {
    #[br(pre_assert("NameProperty" == magic))]
    Name(MapSubNameProperty),
    #[br(pre_assert("StructProperty" == magic))]
    Struct(MapSubStructProperty),
    #[br(pre_assert("FloatProperty" == magic))]
    Float(MapSubFloatProperty),
    #[br(pre_assert("StrProperty" == magic))]
    String(MapSubStrProperty),
    #[br(pre_assert("BoolProperty" == magic))]
    Bool(MapSubBoolProperty),
    #[br(pre_assert("IntProperty" == magic))]
    Int(MapSubIntProperty),
    #[br(pre_assert("EnumProperty" == magic))]
    Enum(MapSubEnumProperty),
}

#[binrw]
#[derive(Debug)]
pub struct GuidStructThing {
    pub guid: [u8; 16],
}

#[binrw]
#[derive(Debug)]
pub struct SomeIDStruct {
    pub guid: [u8; 16],
}

#[binrw]
#[derive(Debug)]
pub struct SomeID2Struct {
    pub guid: [u8; 16],
}

#[binrw]
#[derive(Debug)]
pub struct StringMapKey {
    pub enum_length: u32,
    #[br(count = enum_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub r#enum: String,
}

#[binrw]
#[derive(Debug)]
#[br(import { magic: &KeyType })]
pub enum MapKeyProperty {
    #[br(pre_assert(*magic == KeyType::String))]
    String(StringMapKey),
    #[br(pre_assert(*magic == KeyType::StructMaybe))]
    StructMaybe(StructMaybeKey),
    #[br(pre_assert(*magic == KeyType::Enum))]
    Enum(MapSubEnumProperty),
    #[br(pre_assert(*magic == KeyType::EnumAgain))]
    EnumAgain(MapSubEnumProperty),
    #[br(pre_assert(*magic == KeyType::GUID))]
    GUID(GuidStructThing),
    #[br(pre_assert(*magic == KeyType::SomeID))]
    SomeID(SomeIDStruct),
    #[br(pre_assert(*magic == KeyType::SomeID2))]
    SomeID2(SomeID2Struct),
}

fn is_empty(key: &MapKeyProperty) -> bool {
    false
}

#[binrw]
#[derive(Debug)]
#[br(import(key_type: &KeyType, value_type: &str))]
pub struct MapEntry {
    #[br(args { magic: key_type})]
    #[br(dbg)]
    pub key: MapKeyProperty,

    #[br(args {magic: value_type, is_value: true})]
    #[br(dbg)]
    pub value: MabSubProperty,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum KeyType {
    String = 0x1,
    GUID = 0x3,
    SomeID = 0x6,
    SomeID2 = 0x10,
    EnumAgain = 0x4,
    Enum = 0x7,
    StructMaybe = 0xC,
}

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32, key_type: &KeyType, value_type: &str) -> BinResult<Vec<MapEntry>> {
    let mut result = Vec::<MapEntry>::new();

    let mut current = reader.stream_position().unwrap();
    let end = current + size_in_bytes as u64 - 5 - 3;
    
    while current < end {
        result.push(MapEntry::read_options(reader, endian, (&key_type, &value_type)).unwrap());
        current = reader.stream_position().unwrap();
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
pub struct MapProperty {
    // Plus this int
    #[br(dbg)]
    pub size_in_bytes: u32,

    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_before = 4)]
    pub key_name_length: u32,
    #[br(count = key_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub key_name: String,

    #[br(temp)]
    #[bw(ignore)]
    pub value_name_length: u32,
    #[br(count = value_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub value_name: String,

    #[br(pad_before = 5)]
    #[br(dbg)]
    pub key_type: KeyType,

    #[br(parse_with = custom_parser, args(size_in_bytes, &key_type, &value_name))]
    pub entries: Vec<MapEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_property::MabSubProperty::{Int, Name, String};
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn simple_strmap() {
        // From LocalProfile.sav i think
        let data = [
            0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x0c, 0x00, 0x00, 0x00,
            0x53, 0x74, 0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x41, 0x52, 0x30,
            0x58, 0x4a, 0x47, 0x46, 0x57, 0x41, 0x36, 0x48, 0x4e, 0x49, 0x51, 0x31, 0x41, 0x41,
            0x55, 0x4a, 0x39, 0x55, 0x52, 0x38, 0x32, 0x38, 0x00, 0x07, 0x00, 0x00, 0x00, 0x4e,
            0x41, 0x4d, 0x45, 0x20, 0x31, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = MapProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.size_in_bytes, 49);
        assert_eq!(decoded.key_name, "StrProperty");
        assert_eq!(decoded.value_name, "StrProperty");
        let MapKeyProperty::String(key_property) = &decoded.entries.first().unwrap().key else {
            panic!("StrProperty!")
        };
        let String(value_property) = &decoded.entries.first().unwrap().value else {
            panic!("StrProperty!")
        };
        assert_eq!(key_property.r#enum, "AR0XJGFWA6HNIQ1AAUJ9UR828");
        assert_eq!(value_property.name, "NAME 1");
    }

    #[test]
    fn name_int_map() {
        // From Persistent.sav, EventParams
        let data = [
            0xba, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61,
            0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x0c, 0x00, 0x00,
            0x00, 0x49, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x53, 0x65,
            0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x4d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69,
            0x61, 0x6c, 0x5f, 0x50, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x5f, 0x30, 0x32, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x59, 0x4f,
            0x5f, 0x66, 0x6c, 0x30, 0x30, 0x33, 0x00, 0x01, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x4d, 0x30, 0x31, 0x41, 0x30, 0x31, 0x5f, 0x4e, 0x65, 0x78, 0x74, 0x53, 0x74,
            0x61, 0x67, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x4d, 0x30,
            0x31, 0x41, 0x30, 0x31, 0x5f, 0x47, 0x61, 0x74, 0x65, 0x30, 0x30, 0x31, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x4d, 0x30, 0x31, 0x41, 0x30, 0x31, 0x4d,
            0x6f, 0x6e, 0x6f, 0x72, 0x61, 0x69, 0x6c, 0x30, 0x30, 0x31, 0x54, 0x65, 0x73, 0x74,
            0x30, 0x30, 0x30, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x4d,
            0x30, 0x31, 0x41, 0x30, 0x31, 0x5f, 0x47, 0x61, 0x74, 0x65, 0x30, 0x30, 0x35, 0x00,
            0x01, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = MapProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.size_in_bytes, 186);
        assert_eq!(decoded.key_name, "NameProperty");
        assert_eq!(decoded.value_name, "IntProperty");
        let MapKeyProperty::Enum(key_property) = &decoded.entries.first().unwrap().key else {
            panic!("Name!")
        };
        let Int(value_property) = &decoded.entries.first().unwrap().value else {
            panic!("Int!")
        };
        assert_eq!(key_property.r#enum, "SelectedMachine");
        assert_eq!(value_property.value, 2);
        // TODO: test the rest of the values
    }
}
