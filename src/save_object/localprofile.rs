use crate::property::{BoolProperty, IntProperty, StrProperty, map_property::MapProperty};

/// The object stored in `LocalProfile.sav`.
#[paramacro::serialized_struct("")]
#[derive(Debug)]
pub struct LocalProfileObject {
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
