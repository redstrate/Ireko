use binrw::binrw;

use crate::property::PropertyBase;

#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    pub unk: [u8; 8],
}

impl PropertyBase for DateTimeStruct {
    fn type_name() -> &'static str {
        "StructProperty"
    }

    fn struct_name() -> Option<&'static str> {
        Some("DateTime")
    }

    fn size_in_bytes(&self) -> u32 {
        8
    }
}
