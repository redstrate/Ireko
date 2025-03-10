use binrw::binrw;

use super::PropertyBase;

/// A integer.
///
/// See [the Unreal Engine documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/CoreUObject/UObject/UIntProperty?application_version=4.27).
#[binrw]
#[derive(Debug)]
pub struct IntProperty {
    #[bw(calc = 4)]
    pub size_in_bytes: u32,
    #[brw(pad_before = 5)]
    pub value: u32,
}

impl PropertyBase for IntProperty {
    fn type_name() -> &'static str {
        "IntProperty"
    }

    fn size_in_bytes(&self) -> u32 {
        4 + 5 + 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_zero() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = IntProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, 0);
    }

    #[test]
    fn write_zero() {
        let expected_data: [u8; 13] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let property = IntProperty { value: 0 };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }

    #[test]
    fn integer() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = IntProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, 4);
    }

    #[test]
    fn write_integer() {
        let expected_data: [u8; 13] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        ];
        let property = IntProperty { value: 4 };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }
}
