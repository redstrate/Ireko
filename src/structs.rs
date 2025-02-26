use crate::Property;
use crate::array_property::ArrayProperty;
use crate::bool_property::BoolProperty;
use crate::build_data::DABuildDataStruct;
use crate::common::{read_string_with_length, write_string_with_length};
use crate::float_property::FloatProperty;
use crate::guid::Guid;
use crate::int_property::IntProperty;
use crate::linear_color::LinearColorStruct;
use crate::map_property::MapProperty;
use crate::name_property::NameProperty;
use crate::primary_asset_id::PrimaryAssetIdStruct;
use crate::primary_asset_type::PrimaryAssetTypeStruct;
use crate::str_property::StrProperty;
use binrw::{BinRead, BinResult, binrw};
use std::fmt::Debug;

#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    pub unk: [u8; 8],
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DALoadOptionStruct {
    #[paramacro::serialized_field = "LoadTypes"]
    pub load_types: IntProperty,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct SaveSlotInfoStruct {
    #[paramacro::serialized_field = "Name"]
    pub name: StrProperty,

    #[paramacro::serialized_field = "Timestamp"]
    pub timestamp: DateTimeStruct,

    #[paramacro::serialized_field = "Level"]
    pub level: NameProperty,

    #[paramacro::serialized_field = "Players"]
    pub players: ArrayProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DACharacterCommonStatusStruct {
    pub unk: [u8; 17],
}

// TODO: replace all usage of this with StructField
#[binrw]
#[derive(Debug)]
pub struct PrimaryAssetNameProperty {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(if(property_name != "None"))]
    pub type_name: String,

    #[br(if(property_name != "None"))]
    #[br(args { magic: &type_name, name: &property_name})]
    pub key: Option<Box<Property>>,
}

// TODO: allow specializing into structs for StructProperty, and so on
#[binrw]
#[derive(Debug)]
#[br(import { name: &str, r#type: &str })]
pub struct StructField {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(assert(property_name == name))]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(assert(type_name == r#type))]
    pub type_name: String,

    #[br(if(property_name != "None"))]
    #[br(args { magic: &type_name, name: &property_name })]
    pub key: Option<Box<Property>>,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAModuleItemDataStruct {
    #[paramacro::serialized_field = "ModuleLevel"]
    pub module_level: IntProperty,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAAssembleIdDataStruct {
    #[paramacro::serialized_field = "Hanger"]
    pub hanger: Guid,

    #[paramacro::serialized_field = "Headset"]
    pub headset: Guid,

    #[paramacro::serialized_field = "Mobility"]
    pub mobility: Guid,

    #[paramacro::serialized_field = "Thruster"]
    pub thruster: Guid,

    #[paramacro::serialized_field = "Utility"]
    pub utility: Guid,

    #[paramacro::serialized_field = "PrimaryFrontWeapon"]
    pub primary_front_weapon: Guid,

    #[paramacro::serialized_field = "SecondaryFrontWeapon"]
    pub secondary_front_weapon: Guid,

    #[paramacro::serialized_field = "LeftRearWeapon"]
    pub left_rear_weapon: Guid,

    #[paramacro::serialized_field = "RightRearWeapon"]
    pub right_rear_weapon: Guid,

    #[paramacro::serialized_field = "ColoringData"]
    pub coloring_data: DAMachineColoringDataStruct,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAMachineColoringDataStruct {
    #[paramacro::serialized_field = "Hanger"]
    pub hanger: DAModuleColorStruct,

    #[paramacro::serialized_field = "Headset"]
    pub headset: DAModuleColorStruct,

    #[paramacro::serialized_field = "Mobility"]
    pub mobility: DAModuleColorStruct,

    #[paramacro::serialized_field = "Thruster"]
    pub thruster: DAModuleColorStruct,

    #[paramacro::serialized_field = "Utility"]
    pub utility: DAModuleColorStruct,

    #[paramacro::serialized_field = "PrimaryFrontWeapon"]
    pub primary_front_weapon: DAModuleColorStruct,

    #[paramacro::serialized_field = "SecondaryFrontWeapon"]
    pub secondary_front_weapon: DAModuleColorStruct,

    #[paramacro::serialized_field = "LeftRearWeapon"]
    pub left_rear_weapon: DAModuleColorStruct,

    #[paramacro::serialized_field = "RightRearWeapon"]
    pub right_rear_weapon: DAModuleColorStruct,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAHumanoidColoringDataStruct {
    #[paramacro::serialized_field = "Skin"]
    pub skin: LinearColorStruct,

    #[paramacro::serialized_field = "HairBase"]
    pub hair_base: LinearColorStruct,

    #[paramacro::serialized_field = "HairGradation"]
    pub hair_gradation: LinearColorStruct,

    #[paramacro::serialized_field = "HairHighlight"]
    pub hair_highlight: LinearColorStruct,

    #[paramacro::serialized_field = "HeadOption"]
    pub head_option: LinearColorStruct,

    #[paramacro::serialized_field = "EyeL"]
    pub eye_l: LinearColorStruct,

    #[paramacro::serialized_field = "EyeR"]
    pub eye_r: LinearColorStruct,

    #[paramacro::serialized_field = "BodyMain"]
    pub body_main: LinearColorStruct,

    #[paramacro::serialized_field = "BodySub1"]
    pub body_sub1: LinearColorStruct,

    #[paramacro::serialized_field = "BodySub2"]
    pub body_sub2: LinearColorStruct,

    #[paramacro::serialized_field = "BodySub3"]
    pub body_sub3: LinearColorStruct,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAModuleColorStruct {
    #[paramacro::serialized_field = "Main"]
    pub main: LinearColorStruct,

    #[paramacro::serialized_field = "Sub"]
    pub sub: LinearColorStruct,

    #[paramacro::serialized_field = "Inner"]
    pub inner: LinearColorStruct,

    #[paramacro::serialized_field = "Glow"]
    pub glow: LinearColorStruct,
}

#[binrw]
#[derive(Debug)]
pub struct DATriggerDataStruct {
    pub unk: [u8; 319], // trigger weapon config in game
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DACustomizeAssetIdDataStruct {
    #[paramacro::serialized_field = "Body"]
    pub body: PrimaryAssetIdStruct,

    #[paramacro::serialized_field = "Face"]
    pub face: PrimaryAssetIdStruct,

    #[paramacro::serialized_field = "FrontHair"]
    pub front_hair: PrimaryAssetIdStruct,

    #[paramacro::serialized_field = "BackHair"]
    pub back_hair: PrimaryAssetIdStruct,

    #[paramacro::serialized_field = "ColoringData"]
    pub coloring_data: DAHumanoidColoringDataStruct,

    #[paramacro::serialized_field = "FigureData"]
    pub figure_data: DAHumanoidFigureData,

    #[paramacro::serialized_field = "bInverseFaceMesh"]
    pub inverse_face_mesh: BoolProperty,

    #[paramacro::serialized_field = "bInverseFrontHairMesh"]
    pub inverse_front_hair_mesh: BoolProperty,

    #[paramacro::serialized_field = "bInverseBackHairMesh"]
    pub inverse_back_hair_mesh: BoolProperty,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DAHumanoidFigureData {
    #[paramacro::serialized_field = "BustUp"]
    pub bust_up: FloatProperty,

    #[paramacro::serialized_field = "FatUp"]
    pub fat_up: FloatProperty,

    #[paramacro::serialized_field = "ArmUp"]
    pub arm_up: FloatProperty,

    #[paramacro::serialized_field = "LegUp"]
    pub leg_up: FloatProperty,

    #[paramacro::serialized_field = "WaistUp"]
    pub waist_up: FloatProperty,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DATuningDataStruct {
    #[paramacro::serialized_field = "GrantedTuningPointList"]
    pub granted_tuning_point_list: MapProperty,
}

#[binrw]
#[derive(Debug)]
pub struct SavedBuildData {
    pub build_data: DABuildDataStruct,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct DATuningPointData {
    #[paramacro::serialized_field = "TuningPoint"]
    tuning_point: IntProperty,

    #[paramacro::serialized_field = "MaxTuningPoint"]
    max_tuning_point: IntProperty,
}

#[paramacro::serialized_struct]
#[derive(Debug)]
pub struct TransformStruct {
    #[paramacro::serialized_field = "Rotation"]
    rotation: QuatStruct,

    #[paramacro::serialized_field = "Translation"]
    translation: VectorStruct,

    #[paramacro::serialized_field = "Scale3D"]
    scale: VectorStruct,
}

#[binrw]
#[derive(Debug)]
pub struct QuatStruct {
    // TODO: check if w is actually in front or in the back, this is a guess
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[binrw]
#[derive(Debug)]
pub struct VectorStruct {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub unk: u32,
    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(pad_after = 17)]
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
    println!("{:#?}", prelude);
    if prelude.type_name == "StructProperty" {
        println!("{:#?}", StructPrelude::read_le(reader)?);
    }
    let val = T::read_options(reader, endian, ())?;
    Ok(val)
}
