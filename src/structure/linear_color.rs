use binrw::binrw;

use crate::property::PropertyBase;

#[binrw]
#[derive(Debug)]
pub struct LinearColorStruct {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl PropertyBase for LinearColorStruct {
    fn type_name() -> &'static str {
        "StructProperty"
    }

    fn struct_name() -> Option<&'static str> {
        Some("LinearColor")
    }

    fn size_in_bytes(&self) -> u32 {
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn read_color() {
        let data = [
            0x2a, 0x8e, 0x23, 0x3e, 0x45, 0xa1, 0x85, 0x3e, 0x9f, 0xaa, 0xaa, 0x3e, 0x00, 0x00,
            0x00, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = LinearColorStruct::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.r, 0.159722);
        assert_eq!(decoded.g, 0.260996);
        assert_eq!(decoded.b, 0.333333);
        assert_eq!(decoded.a, 0.0);
    }
}
