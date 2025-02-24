use crate::Property;
use crate::build_data::DABuildDataStruct;
use crate::common::{read_string_with_length, write_string_with_length};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    pub unk: [u8; 25],
}

#[binrw]
#[derive(Debug)]
pub struct DALoadOptionStruct {
    pub unk: [u8; 69], // Contains LoadType property
}

#[binrw]
#[derive(Debug)]
pub struct SaveSlotInfoStruct {
    pub unk: [u8; 17],
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

#[binrw]
#[derive(Debug)]
pub struct DAModuleItemDataStruct {
    #[br(pad_after = 9)] // none
    pub module_level: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAAssembleIdDataStruct {
    pub hanger: PrimaryAssetNameProperty,
    pub headset: PrimaryAssetNameProperty,
    pub mobility: PrimaryAssetNameProperty,
    pub thruster: PrimaryAssetNameProperty,
    pub utility: PrimaryAssetNameProperty,
    pub primary_front_weapon: PrimaryAssetNameProperty,
    pub secondary_front_weapon: PrimaryAssetNameProperty,
    pub left_rear_weapon: PrimaryAssetNameProperty,
    pub right_rear_weapon: PrimaryAssetNameProperty,
    // has none at the end
    #[br(pad_after = 9)]
    pub coloring_data: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAMachineColoringDataStruct {
    pub hanger: PrimaryAssetNameProperty,

    pub headset: PrimaryAssetNameProperty,
    pub mobility: PrimaryAssetNameProperty,
    pub thruster: PrimaryAssetNameProperty,
    pub utility: PrimaryAssetNameProperty,
    pub primary_front_weapon: PrimaryAssetNameProperty,
    pub secondary_front_weapon: PrimaryAssetNameProperty,
    pub left_rear_weapon: PrimaryAssetNameProperty,
    // has none at the end
    #[br(pad_after = 9)]
    pub right_rear_weapon: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAHumanoidColoringDataStruct {
    pub skin: PrimaryAssetNameProperty,
    pub hair_base: PrimaryAssetNameProperty,
    pub hair_gradation: PrimaryAssetNameProperty,
    pub hair_highlight: PrimaryAssetNameProperty,
    pub head_option: PrimaryAssetNameProperty,
    pub eye_l: PrimaryAssetNameProperty,
    pub eye_r: PrimaryAssetNameProperty,
    pub body_main: PrimaryAssetNameProperty,
    pub body_sub1: PrimaryAssetNameProperty,
    pub body_sub2: PrimaryAssetNameProperty,
    // has none at the end
    #[br(pad_after = 9)]
    pub body_sub3: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAModuleColorStruct {
    pub main: PrimaryAssetNameProperty,
    pub sub: PrimaryAssetNameProperty,
    pub inner: PrimaryAssetNameProperty,
    // has none at the end
    #[br(pad_after = 9)]
    pub glow: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DATriggerDataStruct {
    pub unk: [u8; 319], // trigger weapon config in game
}

#[binrw]
#[derive(Debug)]
pub struct DACustomizeAssetIdDataStruct {
    pub body: PrimaryAssetNameProperty,

    pub face: PrimaryAssetNameProperty,

    pub front_hair: PrimaryAssetNameProperty,

    pub back_hair: PrimaryAssetNameProperty,

    pub coloring_data: PrimaryAssetNameProperty,

    pub figure_data: PrimaryAssetNameProperty,

    pub inverse_face_mesh: PrimaryAssetNameProperty,

    pub inverse_front_hair_mesh: PrimaryAssetNameProperty,

    // has none at the end
    #[brw(pad_after = 9)]
    pub inverse_back_hair_mesh: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAHumanoidFigureData {
    pub bust_up: PrimaryAssetNameProperty,

    pub fat_up: PrimaryAssetNameProperty,

    pub arm_up: PrimaryAssetNameProperty,

    pub leg_up: PrimaryAssetNameProperty,
    // has none at the end
    #[brw(pad_after = 9)]
    pub waist_up: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DATuningDataStruct {
    #[brw(pad_after = 9)]
    pub granted_tuning_point_list: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct SavedBuildData {
    pub build_data: DABuildDataStruct,
}

#[binrw]
#[derive(Debug)]
pub struct DATuningPointData {
    #[br(args { name: "TuningPoint", r#type: "IntProperty" })]
    tuning_point: StructField,
    #[br(args { name: "MaxTuningPoint", r#type: "IntProperty" })]
    #[brw(pad_after = 9)] // none
    max_tuning_point: StructField,
}

#[binrw]
#[derive(Debug)]
pub struct TransformStruct {
    #[br(args { name: "Rotation", r#type: "StructProperty" })]
    rotation: StructField,
    #[br(args { name: "Translation", r#type: "StructProperty" })]
    translation: StructField,
    #[br(args { name: "Scale3D", r#type: "StructProperty" })]
    #[brw(pad_after = 9)] // none
    scale: StructField,
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
