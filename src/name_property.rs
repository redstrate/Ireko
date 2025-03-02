use crate::common::{read_string_with_length, write_string_with_length};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct NameProperty {
    #[brw(pad_after = 5)]
    // Only add + 1 for the null terminator if the string *isn't* empty.
    #[bw(calc = value.len() as u32 + 4 + if value.is_empty() { 0 } else { 1})]
    pub size_in_bytes: u32,

    #[br(parse_with = read_string_with_length)]
    #[bw(write_with = write_string_with_length)]
    pub value: String,
}

impl crate::structs::PropertyBase for NameProperty {
    fn type_name() -> &'static str {
        "NameProperty"
    }

    fn size_in_bytes(&self) -> u32 {
        5 + 4 + crate::common::size_of_string_with_length(&self.value)
    }
}
