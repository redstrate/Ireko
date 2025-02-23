use crate::StringBasedProperty;
use crate::common::read_string_with_length;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct SetEntry {
    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub unk_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub unk_type: String,

    #[br(args { magic: &unk_type })]
    pub key: StringBasedProperty,
}

#[binrw]
#[derive(Debug)]
pub struct SetProperty {
    pub unk: u32,

    #[br(pad_before = 4, parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub key_name: String,

    #[br(pad_before = 5)]
    pub num_set_entries: u32,

    #[br(count = num_set_entries)]
    pub entries: Vec<SetEntry>,
}

// TODO: write test
