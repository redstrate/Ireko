use crate::linear_color::LinearColorStruct;

#[paramacro::serialized_struct("DAHumanoidColoringData")]
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
