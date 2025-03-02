pub mod array_property;
pub mod bool_property;
pub mod enum_property;
pub mod float_property;
pub mod generic_property;
pub mod int_property;
pub mod map_property;
pub mod name_property;
pub mod set_property;
pub mod str_property;
pub mod struct_property;

pub(crate) trait PropertyBase {
    fn type_name() -> &'static str;
    fn size_in_bytes(&self) -> u32;

    // these are only relevant for structs:
    // FIXME: this isn't great'
    fn struct_name() -> Option<&'static str> {
        None
    }
}
