use binrw::{BinRead, BinResult, BinWrite, binrw};
use build_data::DABuildDataStruct;
use da_assemble_id_data::DAAssembleIdDataStruct;
use da_customize_asset_id::DACustomizeAssetIdDataStruct;
use da_humanoid_coloring_data::DAHumanoidColoringDataStruct;
use da_humanoid_figure_data::DAHumanoidFigureData;
use da_load_option::DALoadOptionStruct;
use da_machine_coloring_data::DAMachineColoringDataStruct;
use da_module_color::DAModuleColorStruct;
use da_module_item_data::DAModuleItemDataStruct;
use da_trigger_data::DATriggerDataStruct;
use da_tuning_data::DATuningDataStruct;
use da_tuning_point_data::DATuningPointData;
use datetime::DateTimeStruct;
use guid::Guid;
use linear_color::LinearColorStruct;
use primary_asset_id::PrimaryAssetIdStruct;
use primary_asset_type::PrimaryAssetTypeStruct;
use quat::QuatStruct;
use save_slot_info::SaveSlotInfoStruct;
use std::fmt::Debug;
use transform::TransformStruct;
use vector::VectorStruct;

use crate::{
    common::{read_string_with_length, write_string_with_length},
    property::PropertyBase,
};

pub mod build_data;
pub mod da_assemble_id_data;
pub mod da_customize_asset_id;
pub mod da_humanoid_coloring_data;
pub mod da_humanoid_figure_data;
pub mod da_load_option;
pub mod da_machine_coloring_data;
pub mod da_module_color;
pub mod da_module_item_data;
pub mod da_trigger_data;
pub mod da_tuning_data;
pub mod da_tuning_point_data;
pub mod datetime;
pub mod guid;
pub mod linear_color;
pub mod primary_asset_id;
pub mod primary_asset_type;
pub mod quat;
pub mod save_slot_info;
pub mod transform;
pub mod vector;

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
pub struct StructFieldPrelude {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub type_name: String,
}

#[binrw]
#[derive(Debug)]
pub struct StructPrelude {
    pub size_in_bytes: u32,
    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[brw(pad_after = 17)]
    pub struct_name: String,
}

#[binrw::parser(reader, endian)]
pub(crate) fn read_struct_field<T: BinRead<Args<'static> = ()> + Debug>(
    name: &str,
) -> BinResult<T> {
    let prelude = StructFieldPrelude::read_le(reader)?;
    if prelude.property_name != name {
        panic!(
            "Property name doesn't match! Is supposed to be {} but is actually {}",
            name, prelude.property_name
        );
    }
    // TODO: type check with type_name()
    if prelude.type_name == "StructProperty" {
        StructPrelude::read_le(reader)?;
    }
    let val = T::read_options(reader, endian, ())?;
    Ok(val)
}

#[binrw::writer(writer, endian)]
pub(crate) fn write_struct_field<T: PropertyBase + BinWrite<Args<'static> = ()> + Debug>(
    structure: &T,
    name: &str,
) -> BinResult<()> {
    let prelude = StructFieldPrelude {
        property_name: name.to_string(),
        type_name: T::type_name().to_string(),
    };
    prelude.write_le(writer)?;
    if T::type_name() == "StructProperty" {
        let struct_prelude = StructPrelude {
            size_in_bytes: T::size_in_bytes(structure),
            struct_name: T::struct_name().unwrap().to_string(),
        };
        struct_prelude.write_le(writer)?;
    }
    structure.write_options(writer, endian, ())?;
    Ok(())
}

pub(crate) fn calc_struct_field_prelude_byte_size(
    type_name: &str,
    field_name: &str,
    struct_name: Option<&str>,
) -> u32 {
    let mut base_size = crate::common::size_of_string_with_length(field_name);

    // This is an easy way to detect properties that are actually structs
    if struct_name.is_some() {
        base_size += crate::common::size_of_string_with_length("StructProperty")
            + crate::common::size_of_string_with_length(struct_name.unwrap())
            + 4
            + 17
            + 4; // see struct prelude
    } else {
        base_size += crate::common::size_of_string_with_length(type_name);
    }

    base_size
}
