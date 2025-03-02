use crate::{
    property::{BoolProperty, FloatProperty, IntProperty, NameProperty, StrProperty},
    structure::{
        da_load_option::DALoadOptionStruct, datetime::DateTimeStruct,
        save_slot_info::SaveSlotInfoStruct,
    },
};

#[paramacro::serialized_struct("Transform")]
#[derive(Debug)]
pub struct Slot {
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
