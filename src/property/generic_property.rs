use binrw::binrw;

use crate::{
    common::{read_string_with_length, write_string_with_length},
    save_object::generic::Property,
};

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
