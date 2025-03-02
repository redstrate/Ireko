use binrw::binrw;

use crate::property::PropertyBase;

#[binrw]
#[derive(Debug)]
pub struct QuatStruct {
    // TODO: check if w is actually in front or in the back, this is a guess
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PropertyBase for QuatStruct {
    fn type_name() -> &'static str {
        "StructProperty"
    }

    fn struct_name() -> Option<&'static str> {
        Some("Quat")
    }

    fn size_in_bytes(&self) -> u32 {
        16
    }
}
