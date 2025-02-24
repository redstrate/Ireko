use crate::common::{read_string_with_length, write_string_with_length};
use crate::map_property::{KeyType, StringMapKey};
use crate::struct_property::Struct;
use binrw::{BinRead, BinResult, binrw};

fn calculate_header_size(key_name: &str, array_key_data: &ArrayKeyData) -> u64 {
    // TODO: blah, needs to take into account everything before the first item
    match array_key_data {
        ArrayKeyData::String() => 0,
        ArrayKeyData::Struct {
            name,
            type_name,
            struct_name,
        } => {
            // for magic numbers see padding in ArrayKeyData
            // plus null terminator
            (name.len() + type_name.len() + struct_name.len() + 18 + 7 + key_name.len() + 1) as u64
        }
    }
}

#[binrw::parser(reader, endian)]
fn custom_parser(
    size_in_bytes: u32,
    key_name: &str,
    value_type: &KeyType,
    key_data: &ArrayKeyData,
) -> BinResult<Vec<ArrayEntry>> {
    let mut result = Vec::<ArrayEntry>::new();

    let mut current = reader.stream_position()?;
    let end = current + size_in_bytes as u64 - 4 - calculate_header_size(key_name, key_data);

    while current < end {
        result.push(ArrayEntry::read_options(
            reader,
            endian,
            (value_type, key_data),
        )?);
        current = reader.stream_position()?;
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
#[br(import { magic: &KeyType, key_data: &ArrayKeyData })]
pub enum ArrayValue {
    #[br(pre_assert(KeyType::ArrayStruct == *magic))]
    Struct {
        #[br(args { magic: match key_data { ArrayKeyData::Struct{ struct_name, .. } => { struct_name }, _ => { "" }} })]
        r#struct: Struct,
    },
    #[br(pre_assert(KeyType::String == *magic))]
    String(StringMapKey),
}

#[binrw]
#[derive(Debug)]
#[br(import(value_type: &KeyType, key_data: &ArrayKeyData))]
pub struct ArrayEntry {
    #[br(args { magic: value_type, key_data: key_data })]
    pub key: ArrayValue,
}

fn calc_size_in_bytes(entries: &Vec<ArrayEntry>) -> u32 {
    // TODO: stub
    18
}

#[binrw]
#[derive(Debug)]
#[br(import { key_type: &KeyType })]
pub enum ArrayKeyData {
    #[br(pre_assert(*key_type == KeyType::String))]
    String(),
    #[br(pre_assert(*key_type == KeyType::ArrayStruct))]
    Struct {
        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        name: String,

        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        type_name: String,

        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        #[br(pad_before = 8)] // idk
        #[br(pad_after = 17)] // super idk
        struct_name: String,
    },
}

#[binrw]
#[derive(Debug)]
pub struct ArrayProperty {
    #[bw(calc = calc_size_in_bytes(entries))]
    pub size_in_bytes: u32,

    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub key_name: String,

    #[brw(pad_before = 1)]
    pub key_type: KeyType,

    #[br(args { key_type: &key_type })]
    pub key_data: ArrayKeyData,

    #[br(parse_with = custom_parser, args(size_in_bytes, &key_name, &key_type, &key_data))]
    pub entries: Vec<ArrayEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_property::StringMapKey;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_simple_array() {
        // From Slot.sav
        let data = [
            0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x0a, 0x00, 0x00, 0x00, 0x72, 0x65, 0x64, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65,
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = ArrayProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.key_name, "StrProperty");
        let ArrayValue::String(value_property) = &decoded.entries.first().unwrap().key else {
            panic!("StrProperty!")
        };
        assert_eq!(value_property.value, "redstrate");
    }

    #[test]
    fn write_simple_array() {
        let expected_data: [u8; 43] = [
            0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x0a, 0x00, 0x00, 0x00, 0x72, 0x65, 0x64, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65,
            0x00,
        ];
        let property = ArrayProperty {
            key_name: "StrProperty".to_string(),
            key_type: KeyType::String,
            key_data: ArrayKeyData::String(),
            entries: vec![ArrayEntry {
                key: ArrayValue::String(StringMapKey {
                    value: "redstrate".to_string(),
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
    fn read_complex_array() {
        // From Persistent.sav
        let data = [
            0x5f, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00,
            0x0a, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x49, 0x74, 0x65, 0x6d, 0x53, 0x6c,
            0x6f, 0x74, 0x73, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
            0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x0e, 0x08, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
            0x41, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54,
            0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
            0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x37, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
            0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61,
            0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x0b, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x50, 0x6f, 0x74, 0x69,
            0x6f, 0x6e, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74,
            0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50,
            0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x49, 0x74, 0x65, 0x6d, 0x5f, 0x50, 0x6f,
            0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x50, 0x6f, 0x74,
            0x69, 0x6f, 0x6e, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65,
            0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75,
            0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61,
            0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00,
            0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f,
            0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74,
            0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50,
            0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69,
            0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00,
            0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70,
            0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65,
            0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e,
            0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72,
            0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d,
            0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d,
            0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
            0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73,
            0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x35,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69,
            0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00,
            0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79,
            0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
            0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73,
            0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d,
            0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00,
            0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50,
            0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70,
            0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72,
            0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73,
            0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65,
            0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72,
            0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65,
            0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65,
            0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f,
            0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
            0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00,
            0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79,
            0x00, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50,
            0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70,
            0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00,
            0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72,
            0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e,
            0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41,
            0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e,
            0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e,
            0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54,
            0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
            0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
            0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
            0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61,
            0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4e, 0x61,
            0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f,
            0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00,
            0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61,
            0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00,
            0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72,
            0x74, 0x79, 0x00, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54,
            0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d,
            0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70,
            0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e,
            0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72,
            0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00,
            0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00,
            0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e,
            0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65,
            0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75,
            0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61,
            0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00,
            0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f,
            0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74,
            0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50,
            0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69,
            0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00,
            0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70,
            0x65, 0x72, 0x74, 0x79, 0x00, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65,
            0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e,
            0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72,
            0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d,
            0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d,
            0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
            0x79, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = ArrayProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.key_name, "StructProperty");
        assert_eq!(decoded.entries.len(), 10);
    }
}
