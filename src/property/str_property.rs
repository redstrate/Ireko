use crate::common::{read_string_with_length, write_string_with_length};
use binrw::binrw;

use super::PropertyBase;

/// A string.
///
/// See [the Unreal Engine documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/CoreUObject/UObject/UStrProperty?application_version=4.27).
#[binrw]
#[derive(Debug)]
pub struct StrProperty {
    #[brw(pad_after = 5)]
    // Only add + 1 for the null terminator if the string *isn't* empty.
    #[bw(calc = value.len() as u32 + 4 + if value.is_empty() { 0 } else { 1})]
    pub size_in_bytes: u32,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

impl PropertyBase for StrProperty {
    fn type_name() -> &'static str {
        "StrProperty"
    }

    fn size_in_bytes(&self) -> u32 {
        5 + 4 + crate::common::size_of_string_with_length(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_empty_string() {
        // From Slot.sav
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = StrProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, "");
    }

    #[test]
    fn write_empty_string() {
        let expected_data: [u8; 13] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let property = StrProperty {
            value: "".to_string(),
        };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }

    #[test]
    fn read_regular_string() {
        // From Slot.sav
        let data = [
            0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x41,
            0x52, 0x30, 0x58, 0x4a, 0x47, 0x46, 0x57, 0x41, 0x36, 0x48, 0x4e, 0x49, 0x51, 0x31,
            0x41, 0x41, 0x55, 0x4a, 0x39, 0x55, 0x52, 0x38, 0x32, 0x38, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = StrProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, "AR0XJGFWA6HNIQ1AAUJ9UR828");
    }

    #[test]
    fn write_regular_string() {
        let expected_data: [u8; 39] = [
            0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x41,
            0x52, 0x30, 0x58, 0x4a, 0x47, 0x46, 0x57, 0x41, 0x36, 0x48, 0x4e, 0x49, 0x51, 0x31,
            0x41, 0x41, 0x55, 0x4a, 0x39, 0x55, 0x52, 0x38, 0x32, 0x38, 0x00,
        ];
        let property = StrProperty {
            value: "AR0XJGFWA6HNIQ1AAUJ9UR828".to_string(),
        };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }
}
