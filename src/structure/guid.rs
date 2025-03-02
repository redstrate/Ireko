use binrw::binrw;
use std::fmt;

use crate::property::PropertyBase;

// See https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Core/Misc/FGuid
#[binrw]
pub struct Guid {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

impl PropertyBase for Guid {
    fn type_name() -> &'static str {
        "StructProperty"
    }

    fn struct_name() -> Option<&'static str> {
        Some("Guid")
    }

    fn size_in_bytes(&self) -> u32 {
        16
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "{:02X}{:02X}{:02X}{:02X}",
            self.a, self.b, self.c, self.d
        ))
    }
}
