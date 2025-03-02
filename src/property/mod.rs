pub mod array_property;

mod bool_property;
pub use self::bool_property::BoolProperty;

mod enum_property;
pub use self::enum_property::EnumProperty;

mod float_property;
pub use float_property::FloatProperty;

mod generic_property;
pub use self::generic_property::GenericProperty;

mod int_property;
pub use self::int_property::IntProperty;

pub mod map_property;

mod name_property;
pub use self::name_property::NameProperty;

pub mod set_property;

mod str_property;
pub use self::str_property::StrProperty;

mod struct_property;
pub use self::struct_property::StructProperty;

pub(crate) trait PropertyBase {
    fn type_name() -> &'static str;
    fn size_in_bytes(&self) -> u32;

    // these are only relevant for structs:
    // FIXME: this isn't great'
    fn struct_name() -> Option<&'static str> {
        None
    }
}
