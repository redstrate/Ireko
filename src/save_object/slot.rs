use crate::{
    property::{BoolProperty, FloatProperty, IntProperty, NameProperty, StrProperty},
    structure::{DALoadOptionStruct, DateTimeStruct, SaveSlotInfoStruct},
};

/// The object stored in `Slot.sav`.
#[paramacro::serialized_struct("")]
#[derive(Debug)]
pub struct SlotObject {
    #[paramacro::serialized_field = "SavedDataVersion"]
    version: IntProperty,

    #[paramacro::serialized_field = "bDemoVersion"]
    demo: BoolProperty,

    #[paramacro::serialized_field = "CreatedTimeStamp"]
    created_timestamp: DateTimeStruct,

    #[paramacro::serialized_field = "PlayTime"]
    playtime: FloatProperty,

    #[paramacro::serialized_field = "RegisteredName"]
    name: StrProperty,

    #[paramacro::serialized_field = "LoadOption"]
    load_option: DALoadOptionStruct,

    #[paramacro::serialized_field = "DistrictTag"]
    district_tag: NameProperty,

    #[paramacro::serialized_field = "CycleCount"]
    cycle_count: IntProperty,

    #[paramacro::serialized_field = "SlotInfo"]
    slot_info: SaveSlotInfoStruct,
}
