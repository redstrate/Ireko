use crate::{
    bool_property::BoolProperty, int_property::IntProperty, map_property::MapProperty,
    str_property::StrProperty,
};

#[paramacro::serialized_struct("Transform")]
#[derive(Debug)]
pub struct LocalProfile {
    #[paramacro::serialized_field = "SavedDataVersion"]
    version: IntProperty,

    #[paramacro::serialized_field = "bDemoVersion"]
    demo: BoolProperty,

    #[paramacro::serialized_field = "RegisteredNameList"]
    name_list: MapProperty,

    #[paramacro::serialized_field = "SaveGameName"]
    name: StrProperty,

    #[paramacro::serialized_field = "bUseSaveSlot"]
    use_save_slot: BoolProperty,
}
