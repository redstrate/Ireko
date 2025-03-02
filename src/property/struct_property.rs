use crate::{
    common::{read_string_with_length, write_string_with_length},
    structure::Struct,
};
use binrw::binrw;

/// A structure.
///
/// See [the Unreal Engine documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/CoreUObject/UObject/UStructProperty?application_version=4.27).
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
