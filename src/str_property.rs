use crate::common::read_string_with_length;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct StrProperty {
    #[br(pad_after = 5)]
    pub size_in_bytes: u32,

    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn empty_string() {
        // From Slot.sav
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b,
            0x00, 0x00, 0x00, 0x4c, 0x6f, 0x61, 0x64, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = StrProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, "");
    }

    #[test]
    fn regular_string() {
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
}
