use crate::property::IntProperty;

#[paramacro::serialized_struct("DAModuleItemData")]
#[derive(Debug)]
pub struct DAModuleItemDataStruct {
    #[paramacro::serialized_field = "ModuleLevel"]
    pub module_level: IntProperty,
}
