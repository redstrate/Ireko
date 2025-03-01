use crate::common::{read_string_with_length, write_string_with_length};
use crate::map_property::{KeyType, MapSubStrProperty};
use crate::struct_property::Struct;
use binrw::{BinRead, BinResult, binrw};

#[binrw]
#[derive(Debug)]
#[br(import { magic: &str, name: &str })]
pub enum SetValue {
    // NOTE: the name checking is just a temporary workaround
    #[br(pre_assert("StructProperty" == magic && name != "OpenedStrongBoxIds" && name != "AcquiredItemBoxIds"))]
    Struct {
        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        struct_type: String,
        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        r#type: String,
        #[br(parse_with = read_string_with_length)]
        #[bw(write_with = write_string_with_length)]
        #[br(pad_before = 8)]
        struct_type_for_some_reason_again: String,
        #[br(args { magic: &struct_type_for_some_reason_again })]
        #[brw(pad_before = 17)]
        r#struct: Struct,
    },
    #[br(pre_assert("StringProperty" == magic || "NameProperty" == magic))]
    String(MapSubStrProperty),
    #[br(pre_assert("StructProperty" == magic && name == "AcquiredItemBoxIds"))]
    Unknown { unk: [u8; 736] },
    #[br(pre_assert("StructProperty" == magic && name == "OpenedStrongBoxIds"))]
    Unknown2 { unk: [u8; 64] },
}

#[binrw]
#[derive(Debug)]
#[br(import(value_type: &str, key_type: &str))]
pub struct SetEntry {
    #[br(args { magic: value_type, name: key_type })]
    pub key: SetValue,
}

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32, value_type: &str, key_type: &str) -> BinResult<Vec<SetEntry>> {
    let mut result = Vec::<SetEntry>::new();

    let mut current = reader.stream_position()?;
    let end = current + size_in_bytes as u64 - 8;

    while current < end {
        result.push(SetEntry::read_options(
            reader,
            endian,
            (value_type, key_type),
        )?);
        current = reader.stream_position()?;
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
#[br(import(name: &str))]
pub struct SetProperty {
    pub size_in_bytes: u32,

    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub key_name: String,

    #[brw(pad_before = 5)]
    pub key_type: KeyType,

    #[br(parse_with = custom_parser, args(size_in_bytes, &key_name, &name))]
    pub entries: Vec<SetEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn read_struct_set() {
        // From Persistent.sav
        let data = [
            0xc6, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72,
            0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65,
            0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f,
            0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73,
            0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
            0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50,
            0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x42, 0x61, 0x63, 0x6b, 0x48, 0x61, 0x69,
            0x72, 0x50, 0x61, 0x72, 0x74, 0x73, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e,
            0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41,
            0x73, 0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e,
            0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x13, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x49, 0x74, 0x65,
            0x6d, 0x5f, 0x42, 0x48, 0x50, 0x5f, 0x43, 0x48, 0x30, 0x30, 0x35, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69,
            0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00,
            0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70,
            0x65, 0x72, 0x74, 0x79, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65,
            0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e,
            0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72,
            0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x42, 0x61, 0x63, 0x6b, 0x48, 0x61, 0x69, 0x72,
            0x50, 0x61, 0x72, 0x74, 0x73, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73,
            0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61,
            0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x13, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x49, 0x74, 0x65, 0x6d,
            0x5f, 0x42, 0x48, 0x50, 0x5f, 0x43, 0x48, 0x30, 0x30, 0x33, 0x00, 0x05, 0x00, 0x00,
            0x00, 0x4e, 0x6f, 0x6e, 0x65, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = SetProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.key_name, "StructProperty");
    }
}

// TODO: write test
