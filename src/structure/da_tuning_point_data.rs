use crate::property::int_property::IntProperty;

#[paramacro::serialized_struct("DATuningPointData")]
#[derive(Debug)]
pub struct DATuningPointData {
    #[paramacro::serialized_field = "TuningPoint"]
    tuning_point: IntProperty,

    #[paramacro::serialized_field = "MaxTuningPoint"]
    max_tuning_point: IntProperty,
}
