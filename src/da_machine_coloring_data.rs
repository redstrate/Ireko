use crate::da_module_color::DAModuleColorStruct;

#[paramacro::serialized_struct("DAMachineColoringData")]
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
