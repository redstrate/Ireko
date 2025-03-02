use crate::{
    common::{read_string_with_length, write_string_with_length},
    structure::Struct,
};
use binrw::binrw;

/// A structure.
#[binrw]
#[derive(Debug)]
pub struct StructProperty {
    pub unk: u32,
    #[brw(pad_before = 4)]
    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub struct_name: String,
    #[br(args { magic: &struct_name })]
    #[brw(pad_before = 17)]
    pub r#struct: Struct,
}
