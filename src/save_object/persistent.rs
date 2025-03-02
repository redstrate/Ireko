use crate::{
    property::{
        BoolProperty, IntProperty, NameProperty, StrProperty, array_property::ArrayProperty,
        map_property::MapProperty, set_property::SetProperty,
    },
    structure::{DABuildDataStruct, DATuningPointData, TransformStruct},
};

/// The object stored in `Persistent.sav`.
#[paramacro::serialized_struct("")]
#[derive(Debug)]
pub struct PersistentObject {
    #[paramacro::serialized_field = "SavedDataVersion"]
    version: IntProperty,

    #[paramacro::serialized_field = "bDemoVersion"]
    demo_version: BoolProperty,

    #[paramacro::serialized_field = "Money"]
    money: IntProperty,

    #[paramacro::serialized_field = "ObtainedItems"]
    obtained_items: SetProperty,

    #[paramacro::serialized_field = "ItemSlots"]
    item_slots: ArrayProperty,

    #[paramacro::serialized_field = "CurrentItemSlotNum"]
    current_item_slot: IntProperty,

    #[paramacro::serialized_field = "NormalItemInventory"]
    normal_item_inventory: MapProperty,

    #[paramacro::serialized_field = "ModuleInventory"]
    module_inventory: MapProperty,

    #[paramacro::serialized_field = "PartsInventory"]
    parts_inventory: MapProperty,

    #[paramacro::serialized_field = "CurrentBuildData"]
    current_build_data: DABuildDataStruct,

    #[paramacro::serialized_field = "SavedBuildData"]
    saved_build_data: ArrayProperty,

    #[paramacro::serialized_field = "Palettes"]
    palettes: MapProperty,

    #[paramacro::serialized_field = "TuningPointData"]
    tuning_point_data: DATuningPointData,

    #[paramacro::serialized_field = "EventParams"]
    event_params: MapProperty,

    #[paramacro::serialized_field = "ReachedDistricts"]
    reached_districts: SetProperty,

    #[paramacro::serialized_field = "ShopBoughtCount"]
    shop_bought_count: MapProperty,

    #[paramacro::serialized_field = "AcquiredItemBoxIds"]
    acquired_item_box_ids: SetProperty,

    #[paramacro::serialized_field = "OpenedStrongBoxIds"]
    opened_strong_box_ids: SetProperty,

    #[paramacro::serialized_field = "RegressionPlayerStartTag"]
    regression_player_start_tag: NameProperty,

    #[paramacro::serialized_field = "RegressionLevelName"]
    regression_level_name: NameProperty,

    #[paramacro::serialized_field = "bStartFromRegressionPoint"]
    start_from_regression_point: BoolProperty,

    #[paramacro::serialized_field = "ReleasedCheckpoints"]
    released_checkpoints: SetProperty,

    #[paramacro::serialized_field = "SuspendTransform"]
    suspend_transform: TransformStruct,

    #[paramacro::serialized_field = "CharacterPersistentDataList"]
    character_persistent_data_list: MapProperty,

    #[paramacro::serialized_field = "BossStates"]
    boss_states: MapProperty,

    #[paramacro::serialized_field = "NPCStates"]
    npc_states: MapProperty,

    #[paramacro::serialized_field = "ReadDialogues"]
    read_dialogues: MapProperty,

    #[paramacro::serialized_field = "ReadDialogueChains"]
    read_dialogue_chains: MapProperty,

    #[paramacro::serialized_field = "SaveGameName"]
    save_game_name: StrProperty,

    #[paramacro::serialized_field = "bUseSaveSlot"]
    use_save_slot: BoolProperty,
}
