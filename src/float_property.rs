use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FloatProperty {
    #[bw(calc = 4)]
    pub size_in_bytes: u32,
    #[brw(pad_before = 5)]
    pub value: f32,
}

impl crate::structs::PropertyBase for FloatProperty {
    fn type_name() -> &'static str {
        return "FloatProperty";
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
    fn read_value() {
        let data = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x76, 0x9c, 0x45,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = FloatProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, 5006.8184);
    }

    #[test]
    fn write_value() {
        let expected_data: [u8; 13] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x76, 0x9c, 0x45,
        ];
        let property = FloatProperty { value: 5006.8184 };

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            property.write_le(&mut cursor).unwrap();
        }

        assert_eq!(expected_data, &buffer[..]);
    }
}
