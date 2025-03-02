use crate::{
    bool_property::BoolProperty, da_humanoid_coloring_data::DAHumanoidColoringDataStruct,
    da_humanoid_figure_data::DAHumanoidFigureData, primary_asset_id::PrimaryAssetIdStruct,
};

#[paramacro::serialized_struct("DACustomizeAssetIdData")]
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
