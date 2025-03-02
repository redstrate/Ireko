use crate::build_data::DABuildDataStruct;
use crate::common::{read_string_with_length, write_string_with_length};
use crate::guid::Guid;
use crate::linear_color::LinearColorStruct;
use crate::primary_asset_id::PrimaryAssetIdStruct;
use crate::primary_asset_type::PrimaryAssetTypeStruct;
use crate::structs::{
    DAAssembleIdDataStruct, DACustomizeAssetIdDataStruct, DAHumanoidColoringDataStruct,
    DAHumanoidFigureData, DALoadOptionStruct, DAMachineColoringDataStruct, DAModuleColorStruct,
    DAModuleItemDataStruct, DATriggerDataStruct, DATuningDataStruct, DATuningPointData,
    DateTimeStruct, PropertyBase, QuatStruct, SaveSlotInfoStruct, TransformStruct, VectorStruct,
};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
#[br(import { magic: &str })]
pub enum Struct {
    #[br(pre_assert(magic == DateTimeStruct::struct_name().unwrap()))]
    DateTime(DateTimeStruct),
    #[br(pre_assert(magic == DALoadOptionStruct::struct_name().unwrap()))]
    DALoadOption(DALoadOptionStruct),
    #[br(pre_assert(magic == SaveSlotInfoStruct::struct_name().unwrap()))]
    SaveSlotInfo(SaveSlotInfoStruct),
    #[br(pre_assert(magic == PrimaryAssetTypeStruct::struct_name().unwrap()))]
    PrimaryAssetType(PrimaryAssetTypeStruct),
    #[br(pre_assert(magic == PrimaryAssetIdStruct::struct_name().unwrap()))]
    PrimaryAssetId(PrimaryAssetIdStruct),
    #[br(pre_assert(magic == DAModuleItemDataStruct::struct_name().unwrap()))]
    DAModuleItemData(DAModuleItemDataStruct),
    #[br(pre_assert(magic == DABuildDataStruct::struct_name().unwrap()))]
    DABuildData(DABuildDataStruct),
    #[br(pre_assert(magic == DAAssembleIdDataStruct::struct_name().unwrap()))]
    DAAssembleIdData(DAAssembleIdDataStruct),
    #[br(pre_assert(magic == Guid::struct_name().unwrap()))]
    Guid(Guid),
    #[br(pre_assert(magic == DAMachineColoringDataStruct::struct_name().unwrap()))]
    DAMachineColoringData(DAMachineColoringDataStruct),
    #[br(pre_assert(magic == DAModuleColorStruct::struct_name().unwrap()))]
    DAModuleColor(DAModuleColorStruct),
    #[br(pre_assert(magic == LinearColorStruct::struct_name().unwrap()))]
    LinearColor(LinearColorStruct),
    #[br(pre_assert(magic == DATriggerDataStruct::struct_name().unwrap()))]
    DATriggerData(DATriggerDataStruct),
    #[br(pre_assert(magic == DACustomizeAssetIdDataStruct::struct_name().unwrap()))]
    DACustomizeAssetIdData(DACustomizeAssetIdDataStruct),
    #[br(pre_assert(magic == DATuningDataStruct::struct_name().unwrap()))]
    DATuningData(DATuningDataStruct),
    #[br(pre_assert(magic == DAHumanoidColoringDataStruct::struct_name().unwrap()))]
    DAHumanoidColoringData(DAHumanoidColoringDataStruct),
    #[br(pre_assert(magic == DAHumanoidFigureData::struct_name().unwrap()))]
    DAHumanoidFigureData(DAHumanoidFigureData),
    #[br(pre_assert(magic == DATuningPointData::struct_name().unwrap()))]
    DATuningPointData(DATuningPointData),
    #[br(pre_assert(magic == TransformStruct::struct_name().unwrap()))]
    Transform(TransformStruct),
    #[br(pre_assert(magic == QuatStruct::struct_name().unwrap()))]
    Quat(QuatStruct),
    #[br(pre_assert(magic == VectorStruct::struct_name().unwrap()))]
    Vector(VectorStruct),
}

pub(crate) fn calc_size_in_bytes(r#struct: &Struct) -> u32 {
    // todo
    match r#struct {
        Struct::DateTime(date_time_struct) => date_time_struct.size_in_bytes(),
        Struct::DALoadOption(daload_option_struct) => daload_option_struct.size_in_bytes(),
        Struct::SaveSlotInfo(save_slot_info_struct) => save_slot_info_struct.size_in_bytes(),
        Struct::PrimaryAssetType(primary_asset_type_struct) => {
            primary_asset_type_struct.size_in_bytes()
        }
        Struct::PrimaryAssetId(primary_asset_id_struct) => primary_asset_id_struct.size_in_bytes(),
        Struct::DAModuleItemData(damodule_item_data_struct) => {
            damodule_item_data_struct.size_in_bytes()
        }
        Struct::DABuildData(dabuild_data_struct) => dabuild_data_struct.size_in_bytes(),
        Struct::DAAssembleIdData(daassemble_id_data_struct) => {
            daassemble_id_data_struct.size_in_bytes()
        }
        Struct::Guid(guid) => guid.size_in_bytes(),
        Struct::DAMachineColoringData(damachine_coloring_data_struct) => {
            damachine_coloring_data_struct.size_in_bytes()
        }
        Struct::DAModuleColor(damodule_color_struct) => damodule_color_struct.size_in_bytes(),
        Struct::LinearColor(linear_color_struct) => linear_color_struct.size_in_bytes(),
        Struct::DATriggerData(datrigger_data_struct) => datrigger_data_struct.size_in_bytes(),
        Struct::DACustomizeAssetIdData(dacustomize_asset_id_data_struct) => {
            dacustomize_asset_id_data_struct.size_in_bytes()
        }
        Struct::DATuningData(datuning_data_struct) => datuning_data_struct.size_in_bytes(),
        Struct::DAHumanoidColoringData(dahumanoid_coloring_data_struct) => {
            dahumanoid_coloring_data_struct.size_in_bytes()
        }
        Struct::DAHumanoidFigureData(dahumanoid_figure_data) => {
            dahumanoid_figure_data.size_in_bytes()
        }
        Struct::DATuningPointData(datuning_point_data) => datuning_point_data.size_in_bytes(),
        Struct::Transform(transform_struct) => transform_struct.size_in_bytes(),
        Struct::Quat(quat_struct) => quat_struct.size_in_bytes(),
        Struct::Vector(vector_struct) => vector_struct.size_in_bytes(),
    }
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
    #[brw(pad_before = 17)]
    pub r#struct: Struct,
}
