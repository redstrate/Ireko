use crate::property::IntProperty;

#[paramacro::serialized_struct("DALoadOption")]
#[derive(Debug)]
pub struct DALoadOptionStruct {
    #[paramacro::serialized_field = "LoadTypes"]
    pub load_types: IntProperty,
}
