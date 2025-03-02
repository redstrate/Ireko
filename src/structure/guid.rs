use binrw::binrw;
use std::fmt;

use crate::property::PropertyBase;

/// A globally unique identifier.
///
/// See [the Unreal Engine documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Core/Misc/FGuid?application_version=4.27).
#[binrw]
pub struct Guid {
    /// Private.
    pub a: u32,
    /// Holds the second component.
    pub b: u32,
    /// Holds the third component.
    pub c: u32,
    /// Holds the fourth component.
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
