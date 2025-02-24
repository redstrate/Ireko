use crate::common::{read_bool_from, write_bool_as};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct BoolProperty {
    #[brw(pad_before = 8, pad_after = 1)]
    #[br(map = read_bool_from::<u8>)]
    #[bw(map = write_bool_as::<u8>)]
    pub value: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_false() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(data);
        let decoded = BoolProperty::read_le(&mut cursor).unwrap();
        assert!(!decoded.value);
    }

    #[test]
    fn write_false() {
        let expected_data: [u8; 10] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let property = BoolProperty { value: false };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }

    #[test]
    fn read_true() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00];
        let mut cursor = Cursor::new(data);
        let decoded = BoolProperty::read_le(&mut cursor).unwrap();
        assert!(decoded.value);
    }

    #[test]
    fn write_true() {
        let expected_data: [u8; 10] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00];
        let property = BoolProperty { value: true };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }
}
