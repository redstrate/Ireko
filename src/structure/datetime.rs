use binrw::binrw;

use crate::property::PropertyBase;

/// A date and time.
///
/// See [the Unreal Engine documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Core/Misc/FDateTime?application_version=4.27).
#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    /// Number of ticks.
    /// The ticks are 0.1 microseconds (= 100 nanoseconds) since January 1, 0001.
    pub ticks: i64,
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
