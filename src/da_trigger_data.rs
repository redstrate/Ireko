use crate::enum_property::EnumProperty;

#[paramacro::serialized_struct("DATriggerData")]
#[derive(Debug)]
pub struct DATriggerDataStruct {
    #[paramacro::serialized_field = "A"]
    pub a: EnumProperty,

    #[paramacro::serialized_field = "B"]
    pub b: EnumProperty,

    #[paramacro::serialized_field = "C"]
    pub c: EnumProperty,
}
