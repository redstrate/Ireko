use crate::read_bool_from;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct BoolProperty {
    #[br(pad_before = 8, pad_after = 1)]
    #[br(map = read_bool_from::<u8>)]
    #[bw(ignore)]
    pub value: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn r#false() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(data);
        let decoded = BoolProperty::read_le(&mut cursor).unwrap();
        assert!(!decoded.value);
    }

    #[test]
    fn r#true() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00];
        let mut cursor = Cursor::new(data);
        let decoded = BoolProperty::read_le(&mut cursor).unwrap();
        assert!(decoded.value);
    }
}
