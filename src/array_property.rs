use crate::map_property::{MabSubProperty};
use binrw::{binrw, BinRead, BinResult};

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32, value_type: &str) -> BinResult<Vec<ArrayEntry>> {
    let mut result = Vec::<ArrayEntry>::new();

    let mut current = reader.stream_position().unwrap();
    let end = current + size_in_bytes as u64 - 4;

    while current < end {
        println!("current: {}, end: {}", current, end);
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
    #[br(dbg)]
    pub key: MabSubProperty,
}

#[binrw]
#[derive(Debug)]
pub struct ArrayProperty {
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
    #[br(dbg)]
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
            0x12, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x0c,
            0x00, 0x00, 0x00,
            0x53, 0x74, 0x72,
            0x50, 0x72, 0x6f,
            0x70, 0x65, 0x72,
            0x74, 0x79, 0x00,
            0x00, 0x01, 0x00,
            0x00, 0x00, 0x0a,
            0x00, 0x00, 0x00,
            0x72, 0x65, 0x64,
            0x73, 0x74, 0x72,
            0x61, 0x74, 0x65,
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = ArrayProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.size_in_bytes, 18);
        assert_eq!(decoded.key_name, "StrProperty");
        let String(value_property) = &decoded.entries.first().unwrap().key else {
            panic!("StrProperty!")
        };
        assert_eq!(value_property.name, "redstrate");
    }
}
