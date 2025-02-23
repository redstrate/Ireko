use crate::StringBasedProperty;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct SetEntry {
    #[br(temp)]
    #[bw(ignore)]
    pub unk_name_length: u32,
    #[br(count = unk_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub unk_name: String,

    #[br(temp)]
    #[bw(ignore)]
    pub unk_type_length: u32,
    #[br(count = unk_type_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub unk_type: String,

    #[br(args { magic: &unk_type })]

    pub key: StringBasedProperty,
}

#[binrw]
#[derive(Debug)]
pub struct SetProperty {
    pub unk: u32,
    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_before = 4)]
    pub key_name_length: u32,
    #[br(count = key_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub key_name: String,

    #[br(pad_before = 5)]
    pub num_set_entries: u32,

    #[br(count = num_set_entries)]
    pub entries: Vec<SetEntry>,
}

// TODO: write test
