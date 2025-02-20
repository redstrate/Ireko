use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct NameProperty {
    #[br(temp)]
    #[bw(ignore)]
    #[br(pad_after = 5)]
    pub property_name_length: u32,
    #[br(count = property_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub property_name: String,
}
