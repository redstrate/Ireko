use crate::set_property::SetEntry;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct ArrayProperty {
    pub unk: u32,
    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_before = 4)]
    pub key_name_length: u32,
    #[br(count = key_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub key_name: String,

    #[br(pad_before = 1)]
    pub num_array_entries: u32,

    #[br(count = num_array_entries, args { inner: (&*key_name,) })]
    pub entries: Vec<SetEntry>,
}
