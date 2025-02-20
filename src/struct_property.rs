use crate::structs::{
    DAAssembleIdDataStruct, DABuildDataStruct, DACharacterCommonStatusStruct, DALoadOptionStruct,
    DAMachineColoringDataStruct, DAModuleColorStruct, DAModuleItemDataStruct, DateTimeStruct,
    GuidStruct, LinearColorStruct, ParamsStruct, PrimaryAssetIdStruct, PrimaryAssetTypeStruct,
    SaveSlotInfoStruct,
};
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
    PrimaryAssetType(PrimaryAssetTypeStruct),
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
