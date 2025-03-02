use crate::common::{
    read_bool_from, read_string_with_length, write_bool_as, write_string_with_length,
};
use crate::guid::Guid;
use crate::struct_property::{Struct, calc_size_in_bytes};
use crate::structs::{PrimaryAssetNameProperty, PropertyBase};
use binrw::{BinRead, BinResult, binrw};

// parse until we can't parse no more. kind of a hack for how we run into the end of Persistent.Sav
#[binrw::parser(reader, endian)]
fn cc() -> BinResult<Vec<PrimaryAssetNameProperty>> {
    let mut result = Vec::<PrimaryAssetNameProperty>::new();

    loop {
        if let Ok(str) = PrimaryAssetNameProperty::read_options(reader, endian, ()) {
            result.push(str);
        } else {
            break;
        }
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
pub struct MapSubStructProperty {
    #[br(parse_with = cc)]
    fields: Vec<PrimaryAssetNameProperty>,
}

#[binrw]
#[derive(Debug)]
pub struct StructMaybeKey {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub unk_name: String,

    #[brw(pad_after = 8)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub unk_type: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub struct_name: String,

    #[br(args { magic: &struct_name })]
    #[brw(pad_before = 17)]
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
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubStrProperty {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

#[binrw]
#[derive(Debug)]
pub struct MapSubBoolProperty {
    #[br(map = read_bool_from::<u8>)]
    #[bw(map = write_bool_as::<u8>)]
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
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

// Used in MapProperty exclusively, seems to be a shortened version of some Properties
#[binrw]
#[derive(Debug)]
#[br(import { magic: &str })]
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
pub struct StringMapKey {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
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
    GUID(Guid),
    #[br(pre_assert(*magic == KeyType::SomeID))]
    SomeID(Guid),
    #[br(pre_assert(*magic == KeyType::SomeID2))]
    SomeID2(Guid),
    #[br(pre_assert(*magic == KeyType::ArrayStruct))]
    ArrayStruct(StructMaybeKey),
    #[br(pre_assert(*magic == KeyType::Unknown2))]
    SomeID3(Guid),
    #[br(pre_assert(*magic == KeyType::SoftObjectProperty))]
    SoftObjectProperty(StringMapKey),
}

#[binrw]
#[derive(Debug)]
#[br(import(key_type: &KeyType, value_type: &str))]
pub struct MapEntry {
    #[br(args { magic: key_type})]
    pub key: MapKeyProperty,

    #[br(args { magic: value_type })]
    pub value: MabSubProperty,
}

// TODO: im worried that set/array/map actually share same of the same values...
#[binrw]
#[brw(repr = u32)]
#[derive(Debug, PartialEq, Clone)]
pub enum KeyType {
    None = 0x0,
    String = 0x1,
    SetStruct = 0x2,
    SetStruct2 = 0x2f,
    GUID = 0x3,
    // Seen in ReadDialogues
    SoftObjectProperty = 0x5,
    SomeID = 0x6,
    SomeID2 = 0x10,
    EnumAgain = 0x4,
    Enum = 0x7,
    StructMaybe = 0xC,
    ArrayStruct = 0xA, // Only used in ArrayProperty
    Unknown = 0x2E,    // AcquiredItemBoxIds in Persistent.sav
    Unknown2 = 0x33,   // CharacterPersistentDataList in Persistent.sav
}

#[binrw::parser(reader, endian)]
fn custom_parser(
    size_in_bytes: u32,
    key_type: &KeyType,
    value_type: &str,
) -> BinResult<Vec<MapEntry>> {
    let mut result = Vec::<MapEntry>::new();

    let mut current = reader.stream_position()?;
    let end = current + size_in_bytes as u64 - 5 - 3;

    while current < end {
        result.push(MapEntry::read_options(
            reader,
            endian,
            (key_type, value_type),
        )?);
        current = reader.stream_position()?;
    }
    Ok(result)
}

fn calc_entry_size_in_bytes(prop: &MapProperty) -> u32 {
    // TODO: complete guesswork still
    let mut size = 4;

    for entry in &prop.entries {
        size += match &entry.key {
            MapKeyProperty::String(string_map_key) => {
                crate::common::size_of_string_with_length(&string_map_key.value)
            }
            MapKeyProperty::StructMaybe(struct_maybe_key) => {
                crate::common::size_of_string_with_length(&struct_maybe_key.unk_name)
                    + 8
                    + crate::common::size_of_string_with_length(&struct_maybe_key.unk_type)
                    + crate::common::size_of_string_with_length(&struct_maybe_key.struct_name)
                    + 17
                    + crate::struct_property::calc_size_in_bytes(&struct_maybe_key.r#struct)
            }
            MapKeyProperty::Enum(map_sub_enum_property) => {
                crate::common::size_of_string_with_length(&map_sub_enum_property.value)
            }
            MapKeyProperty::EnumAgain(map_sub_enum_property) => {
                crate::common::size_of_string_with_length(&map_sub_enum_property.value)
            }
            MapKeyProperty::GUID(guid) => guid.size_in_bytes(),
            MapKeyProperty::SomeID(guid) => guid.size_in_bytes(),
            MapKeyProperty::SomeID2(guid) => guid.size_in_bytes(),
            MapKeyProperty::ArrayStruct(struct_maybe_key) => {
                crate::common::size_of_string_with_length(&struct_maybe_key.unk_name)
                    + 8
                    + crate::common::size_of_string_with_length(&struct_maybe_key.unk_type)
                    + crate::common::size_of_string_with_length(&struct_maybe_key.struct_name)
                    + 17
                    + crate::struct_property::calc_size_in_bytes(&struct_maybe_key.r#struct)
            }
            MapKeyProperty::SomeID3(guid) => guid.size_in_bytes(),
            MapKeyProperty::SoftObjectProperty(string_map_key) => {
                crate::common::size_of_string_with_length(&string_map_key.value)
            }
        };
        size += match &entry.value {
            MabSubProperty::Name(map_sub_name_property) => {
                crate::common::size_of_string_with_length(&map_sub_name_property.value)
            }
            MabSubProperty::Struct(map_sub_struct_property) => 0,
            MabSubProperty::Float(map_sub_float_property) => 4,
            MabSubProperty::String(map_sub_str_property) => {
                crate::common::size_of_string_with_length(&map_sub_str_property.value)
            }
            MabSubProperty::Bool(map_sub_bool_property) => 1,
            MabSubProperty::Int(map_sub_int_property) => 4,
            MabSubProperty::Enum(map_sub_enum_property) => {
                crate::common::size_of_string_with_length(&map_sub_enum_property.value)
            }
        }
    }

    size
}

#[binrw]
#[derive(Debug)]
pub struct MapProperty {
    #[bw(calc = calc_entry_size_in_bytes(&self))]
    pub size_in_bytes: u32,

    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub key_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value_name: String,

    #[brw(pad_before = 5)]
    #[br(dbg)]
    pub map_key_type: KeyType,

    #[br(parse_with = custom_parser, args(size_in_bytes, &map_key_type, &value_name))]
    pub entries: Vec<MapEntry>,
}

impl crate::structs::PropertyBase for MapProperty {
    fn type_name() -> &'static str {
        return "MapProperty";
    }

    fn size_in_bytes(&self) -> u32 {
        4 + 4
            + crate::common::size_of_string_with_length(&self.key_name)
            + crate::common::size_of_string_with_length(&self.value_name)
            + 5
            + 4
            + calc_entry_size_in_bytes(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_property::MabSubProperty::{Int, String};
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_simple_strmap() {
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
        assert_eq!(decoded.key_name, "StrProperty");
        assert_eq!(decoded.value_name, "StrProperty");
        let MapKeyProperty::String(key_property) = &decoded.entries.first().unwrap().key else {
            panic!("StrProperty!")
        };
        let String(value_property) = &decoded.entries.first().unwrap().value else {
            panic!("StrProperty!")
        };
        assert_eq!(key_property.value, "AR0XJGFWA6HNIQ1AAUJ9UR828");
        assert_eq!(value_property.value, "NAME 1");
    }

    #[test]
    fn write_simple_map() {
        let expected_data: [u8; 90] = [
            0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x0c, 0x00, 0x00, 0x00,
            0x53, 0x74, 0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x41, 0x52, 0x30,
            0x58, 0x4a, 0x47, 0x46, 0x57, 0x41, 0x36, 0x48, 0x4e, 0x49, 0x51, 0x31, 0x41, 0x41,
            0x55, 0x4a, 0x39, 0x55, 0x52, 0x38, 0x32, 0x38, 0x00, 0x07, 0x00, 0x00, 0x00, 0x4e,
            0x41, 0x4d, 0x45, 0x20, 0x31, 0x00,
        ];
        let property = MapProperty {
            key_name: "StrProperty".to_string(),
            value_name: "StrProperty".to_string(),
            map_key_type: KeyType::String,
            entries: vec![MapEntry {
                key: MapKeyProperty::String(StringMapKey {
                    value: "AR0XJGFWA6HNIQ1AAUJ9UR828".to_string(),
                }),
                value: String(MapSubStrProperty {
                    value: "NAME 1".to_string(),
                }),
            }],
        };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
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
        assert_eq!(decoded.key_name, "NameProperty");
        assert_eq!(decoded.value_name, "IntProperty");
        let MapKeyProperty::Enum(key_property) = &decoded.entries.first().unwrap().key else {
            panic!("Name!")
        };
        let Int(value_property) = &decoded.entries.first().unwrap().value else {
            panic!("Int!")
        };
        assert_eq!(key_property.value, "SelectedMachine");
        assert_eq!(value_property.value, 2);
        // TODO: test the rest of the values
    }
}
