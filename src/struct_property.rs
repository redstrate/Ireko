use crate::structs::{CarryCountProperty, DAAssembleIdDataStruct, DABuildDataStruct, DACharacterCommonStatusStruct, DALoadOptionStruct, DAMachineColoringDataStruct, DAModuleColorStruct, DAModuleItemDataStruct, DateTimeStruct, GuidStruct, LinearColorStruct, ParamsStruct, PrimaryAssetIdStruct, PrimaryAssetNameProperty, SaveSlotInfoStruct};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub enum Struct {
    #[br(magic = b"DateTime\0")]
    DateTime(DateTimeStruct),
    #[br(magic = b"DALoadOption\0")]
    DALoadOption(DALoadOptionStruct),
    #[br(magic = b"SaveSlotInfo\0")]
    SaveSlotInfo(SaveSlotInfoStruct),
    #[br(magic = b"DACharacterCommonStatus\0")]
    DACharacterCommonStatus(DACharacterCommonStatusStruct),
    #[br(magic = b"Params\0")]
    Params(ParamsStruct),
    #[br(magic = b"PrimaryAssetType\0")]
    PrimaryAssetType {
        #[br(pad_before = 17)]
        #[br(dbg)]
        name: PrimaryAssetNameProperty,
        #[br(pad_before = 9)] // "None" and it's length in bytes plus the null terminator
        #[br(pad_after = 9)] // ditto
        #[br(dbg)]
        primary_asset_name: PrimaryAssetNameProperty,
    },
    #[br(magic = b"PrimaryAssetId\0")]
    PrimaryAssetId(PrimaryAssetIdStruct),
    #[br(magic = b"DAModuleItemData\0")]
    DAModuleItemData(DAModuleItemDataStruct),
    #[br(magic = b"DABuildData\0")]
    DABuildData(DABuildDataStruct),
    #[br(magic = b"DAAssembleIdData\0")]
    DAAssembleIdData(DAAssembleIdDataStruct),
    #[br(magic = b"Guid\0")]
    Guid(GuidStruct),
    #[br(magic = b"DAMachineColoringData\0")]
    DAMachineColoringData(DAMachineColoringDataStruct),
    #[br(magic = b"DAModuleColor\0")]
    DAModuleColor(DAModuleColorStruct),
    #[br(magic = b"LinearColor\0")]
    LinearColor(LinearColorStruct),
    #[br(magic = b"CarryCount\0")]
    CarryCount {
        #[br(dbg)]
        carry_count: CarryCountProperty,
        #[br(dbg)]
        #[br(pad_before = 15)] // "StoreCount" + 4 bytes for length + 1 byte for endofstring
        #[br(pad_after = 9)] // "None" + 1 byte for endofstring + 4 bytes for length
        store_count: CarryCountProperty,
    },
    #[br(magic = b"Map\0")]
    Map {
        #[br(dbg)]
        #[br(pad_after = 9)] // "None" + 1 byte for endofstring + 4 bytes for length
        map: CarryCountProperty,
    },
    // TODO: im almost certain this isn't a struct name
    #[br(magic = b"ID\0")]
    ID {
        unk: [u8; 149], // not sure how to parse this yet
        #[br(dbg)]
        #[br(pad_after = 9)] // "None" and it's length in bytes plus the null terminator
        name: PrimaryAssetNameProperty,

        #[br(dbg)]
        #[br(pad_after = 9)] // "None" and it's length in bytes plus the null terminator
        primary_asset_name: PrimaryAssetNameProperty,

        #[br(dbg)]
        data: [u8; 137],
    },
    #[br(magic = b"Set\0")]
    Set {
        #[br(dbg)]
        #[br(pad_after = 9)] // "None" + 1 byte for endofstring + 4 bytes for length
        set: CarryCountProperty,
    },
}

#[binrw]
#[derive(Debug)]
pub struct StructProperty {
    pub unk: u32,
    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_before = 4)]
    pub name_length: u32,
    pub r#struct: Struct,
}
