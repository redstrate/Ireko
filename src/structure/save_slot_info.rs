use crate::property::{
    array_property::ArrayProperty, name_property::NameProperty, str_property::StrProperty,
};

use super::datetime::DateTimeStruct;

#[paramacro::serialized_struct("SaveSlotInfo")]
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
