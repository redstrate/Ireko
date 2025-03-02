use super::linear_color::LinearColorStruct;

#[paramacro::serialized_struct("DAModuleColor")]
#[derive(Debug)]
pub struct DAModuleColorStruct {
    #[paramacro::serialized_field = "Main"]
    pub main: LinearColorStruct,

    #[paramacro::serialized_field = "Sub"]
    pub sub: LinearColorStruct,

    #[paramacro::serialized_field = "Inner"]
    pub inner: LinearColorStruct,

    #[paramacro::serialized_field = "Glow"]
    pub glow: LinearColorStruct,
}
