use crate::float_property::FloatProperty;

#[paramacro::serialized_struct("DAHumanoidFigureData")]
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
