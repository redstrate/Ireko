use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FloatProperty {
    /// Typically 4 for f32
    pub byte_size: u32,
    #[br(pad_before = 5)]
    pub value: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn value() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x76, 0x9c, 0x45,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = FloatProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.byte_size, 4);
        assert_eq!(decoded.value, 5006.8184);
    }
}
