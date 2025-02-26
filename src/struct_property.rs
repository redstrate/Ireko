use crate::build_data::DABuildDataStruct;
use crate::common::{read_string_with_length, write_string_with_length};
use crate::guid::Guid;
use crate::linear_color::LinearColorStruct;
use crate::primary_asset_id::PrimaryAssetIdStruct;
use crate::primary_asset_type::PrimaryAssetTypeStruct;
use crate::structs::{
    DAAssembleIdDataStruct, DACustomizeAssetIdDataStruct,
    DAHumanoidColoringDataStruct, DAHumanoidFigureData, DALoadOptionStruct,
    DAMachineColoringDataStruct, DAModuleColorStruct, DAModuleItemDataStruct, DATriggerDataStruct,
    DATuningDataStruct, DATuningPointData, DateTimeStruct, QuatStruct, SaveSlotInfoStruct,
    TransformStruct, VectorStruct,
};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
#[br(import { magic: &str })]
pub enum Struct {
    #[br(pre_assert(magic == "DateTime"))]
    DateTime(DateTimeStruct),
    #[br(pre_assert(magic == "DALoadOption"))]
    DALoadOption(DALoadOptionStruct),
    #[br(pre_assert(magic == "SaveSlotInfo"))]
    SaveSlotInfo(SaveSlotInfoStruct),
    #[br(pre_assert(magic == "PrimaryAssetType"))]
    PrimaryAssetType(PrimaryAssetTypeStruct),
    #[br(pre_assert(magic == "PrimaryAssetId"))]
    PrimaryAssetId(PrimaryAssetIdStruct),
    #[br(pre_assert(magic == "DAModuleItemData"))]
    DAModuleItemData(DAModuleItemDataStruct),
    #[br(pre_assert(magic == "DABuildData"))]
    DABuildData(DABuildDataStruct),
    #[br(pre_assert(magic == "DAAssembleIdData"))]
    DAAssembleIdData(DAAssembleIdDataStruct),
    #[br(pre_assert(magic == "Guid"))]
    Guid(Guid),
    #[br(pre_assert(magic == "DAMachineColoringData"))]
    DAMachineColoringData(DAMachineColoringDataStruct),
    #[br(pre_assert(magic == "DAModuleColor"))]
    DAModuleColor(DAModuleColorStruct),
    #[br(pre_assert(magic == "LinearColor"))]
    LinearColor(LinearColorStruct),
    #[br(pre_assert(magic == "DATriggerData"))]
    DATriggerData(DATriggerDataStruct),
    #[br(pre_assert(magic == "DACustomizeAssetIdData"))]
    DACustomizeAssetIdData(DACustomizeAssetIdDataStruct),
    #[br(pre_assert(magic == "DATuningData"))]
    DATuningData(DATuningDataStruct),
    #[br(pre_assert(magic == "DAHumanoidColoringData"))]
    DAHumanoidColoringData(DAHumanoidColoringDataStruct),
    #[br(pre_assert(magic == "DAHumanoidFigureData"))]
    DAHumanoidFigureData(DAHumanoidFigureData),
    #[br(pre_assert(magic == "DATuningPointData"))]
    DATuningPointData(DATuningPointData),
    #[br(pre_assert(magic == "Transform"))]
    Transform(TransformStruct),
    #[br(pre_assert(magic == "Quat"))]
    Quat(QuatStruct),
    #[br(pre_assert(magic == "Vector"))]
    Vector(VectorStruct),
}

#[binrw]
#[derive(Debug)]
pub struct StructProperty {
    pub unk: u32,
    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub struct_name: String,
    #[br(args { magic: &struct_name })]
    #[br(pad_before = 17)]
    pub r#struct: Struct,
}
