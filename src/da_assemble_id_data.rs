use crate::{da_machine_coloring_data::DAMachineColoringDataStruct, guid::Guid};

#[paramacro::serialized_struct("DAAssembleIdData")]
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
