use crate::set_property::SetEntry;
use binrw::{binrw, BinRead, BinResult};

#[binrw::parser(reader, endian)]
fn custom_parser(size_in_bytes: u32) -> BinResult<Vec<SetEntry>> {
    let mut result = Vec::<SetEntry>::new();

    let mut current = reader.stream_position().unwrap();
    let end = current + size_in_bytes as u64 - 4;

    while current < end {
        result.push(SetEntry::read_options(reader, endian, ()).unwrap());
        current = reader.stream_position().unwrap();
    }
    Ok(result)
}

#[binrw]
#[derive(Debug)]
pub struct ArrayProperty {
    // Plus this int
    pub size_in_bytes: u32,

    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_before = 4)]
    pub key_name_length: u32,
    #[br(count = key_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub key_name: String,

    #[br(pad_before = 1)]
    pub unk: u32,

    #[br(parse_with = custom_parser, args(size_in_bytes))]
    pub entries: Vec<SetEntry>,
}
