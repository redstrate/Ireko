use crate::common::read_string_with_length;
use crate::map_property::MabSubProperty;
use binrw::{BinRead, BinResult, binrw};

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32, value_type: &str) -> BinResult<Vec<ArrayEntry>> {
    let mut result = Vec::<ArrayEntry>::new();

    let mut current = reader.stream_position().unwrap();
    let end = current + size_in_bytes as u64 - 4;

    while current < end {
        result.push(ArrayEntry::read_options(reader, endian, (value_type,)).unwrap());
        current = reader.stream_position().unwrap();
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
#[br(import(value_type: &str))]
pub struct ArrayEntry {
    #[br(args { magic: &value_type })]
    pub key: MabSubProperty,
}

#[binrw]
#[derive(Debug)]
pub struct ArrayProperty {
    // Plus this int
    pub size_in_bytes: u32,

    #[br(pad_before = 4, parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub key_name: String,

    #[br(pad_before = 1)]
    pub unk: u32,

    #[br(parse_with = custom_parser, args(size_in_bytes, &key_name))]
    pub entries: Vec<ArrayEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_property::MabSubProperty::String;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn simple_array() {
        // From Slot.sav
        let data = [
            0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x53, 0x74,
            0x72, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x0a, 0x00, 0x00, 0x00, 0x72, 0x65, 0x64, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65,
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = ArrayProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.size_in_bytes, 18);
        assert_eq!(decoded.key_name, "StrProperty");
        let String(value_property) = &decoded.entries.first().unwrap().key else {
            panic!("StrProperty!")
        };
        assert_eq!(value_property.value, "redstrate");
    }
}
