use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct StrProperty {
    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_after = 5)]
    pub name_length: u32,
    #[br(count = name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub name: String,
}
