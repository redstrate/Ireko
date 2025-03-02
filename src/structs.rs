use crate::Property;
use crate::common::{read_string_with_length, write_string_with_length};
use binrw::{BinRead, BinResult, BinWrite, binrw};
use std::fmt::Debug;

#[binrw]
#[derive(Debug)]
pub struct GenericProperty {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[br(if(property_name != "None"))]
    pub type_name: String,

    #[br(if(property_name != "None"))]
    #[br(args { magic: &type_name, name: &property_name})]
    pub key: Option<Box<Property>>,
}

#[binrw]
#[derive(Debug)]
pub struct StructFieldPrelude {
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub type_name: String,
}

#[binrw]
#[derive(Debug)]
pub struct StructPrelude {
    pub size_in_bytes: u32,
    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    #[brw(pad_after = 17)]
    pub struct_name: String,
}

#[binrw::parser(reader, endian)]
pub(crate) fn read_struct_field<T: BinRead<Args<'static> = ()> + Debug>(
    name: &str,
) -> BinResult<T> {
    let prelude = StructFieldPrelude::read_le(reader)?;
    if prelude.property_name != name {
        panic!(
            "Property name doesn't match! Is supposed to be {} but is actually {}",
            name, prelude.property_name
        );
    }
    // TODO: type check with type_name()
    if prelude.type_name == "StructProperty" {
        StructPrelude::read_le(reader)?;
    }
    let val = T::read_options(reader, endian, ())?;
    Ok(val)
}

pub(crate) trait PropertyBase {
    fn type_name() -> &'static str;
    fn size_in_bytes(&self) -> u32;

    // these are only relevant for structs:
    // FIXME: this isn't great'
    fn struct_name() -> Option<&'static str> {
        None
    }
}

#[binrw::writer(writer, endian)]
pub(crate) fn write_struct_field<T: PropertyBase + BinWrite<Args<'static> = ()> + Debug>(
    structure: &T,
    name: &str,
) -> BinResult<()> {
    let prelude = StructFieldPrelude {
        property_name: name.to_string(),
        type_name: T::type_name().to_string(),
    };
    prelude.write_le(writer)?;
    if T::type_name() == "StructProperty" {
        let struct_prelude = StructPrelude {
            size_in_bytes: T::size_in_bytes(structure),
            struct_name: T::struct_name().unwrap().to_string(),
        };
        struct_prelude.write_le(writer)?;
    }
    structure.write_options(writer, endian, ())?;
    Ok(())
}

pub(crate) fn calc_struct_field_prelude_byte_size(
    type_name: &str,
    field_name: &str,
    struct_name: Option<&str>,
) -> u32 {
    let mut base_size = crate::common::size_of_string_with_length(field_name);

    // This is an easy way to detect properties that are actually structs
    if struct_name.is_some() {
        base_size += crate::common::size_of_string_with_length("StructProperty")
            + crate::common::size_of_string_with_length(struct_name.unwrap())
            + 4
            + 17
            + 4; // see struct prelude
    } else {
        base_size += crate::common::size_of_string_with_length(type_name);
    }

    base_size
}
