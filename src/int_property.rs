use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct IntProperty {
    /// In bytes
    pub integer_size: u32,
    #[br(pad_before = 5)]
    pub value: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn zero() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = IntProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.integer_size, 4);
        assert_eq!(decoded.value, 0);
    }

    #[test]
    fn integer() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = IntProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.integer_size, 4);
        assert_eq!(decoded.value, 4);
    }
}
