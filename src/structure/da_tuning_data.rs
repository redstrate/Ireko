use crate::property::map_property::MapProperty;

#[paramacro::serialized_struct("DATuningData")]
#[derive(Debug)]
pub struct DATuningDataStruct {
    #[paramacro::serialized_field = "GrantedTuningPointList"]
    pub granted_tuning_point_list: MapProperty,
}
