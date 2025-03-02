use binrw::binrw;

use crate::structs::PropertyBase;

#[binrw]
#[derive(Debug)]
pub struct VectorStruct {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PropertyBase for VectorStruct {
    fn type_name() -> &'static str {
        "StructProperty"
    }

    fn struct_name() -> Option<&'static str> {
        Some("Vector")
    }

    fn size_in_bytes(&self) -> u32 {
        16
    }
}
