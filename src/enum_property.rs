use crate::common::{read_string_with_length, write_string_with_length};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct EnumProperty {
    #[br(pad_before = 8)] // unk
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub enum_type: String,

    #[br(pad_before = 1)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

impl crate::structs::PropertyBase for EnumProperty {
    fn type_name() -> &'static str {
        return "EnumProperty";
    }

    fn size_in_bytes(&self) -> u32 {
        8 + crate::common::size_of_string_with_length(&self.enum_type)
            + 1
            + crate::common::size_of_string_with_length(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn read_enum() {
        // Persistent.sav
        let data = [
            0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x45, 0x44,
            0x41, 0x57, 0x65, 0x61, 0x70, 0x6f, 0x6e, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x50,
            0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00, 0x45,
            0x44, 0x41, 0x57, 0x65, 0x61, 0x70, 0x6f, 0x6e, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65,
            0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3a, 0x46, 0x72, 0x6f, 0x6e,
            0x74, 0x57, 0x65, 0x61, 0x70, 0x6f, 0x6e, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = EnumProperty::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.enum_type, "EDAWeaponModulePosition");
        assert_eq!(decoded.value, "EDAWeaponModulePosition::FrontWeapon");
    }
}
