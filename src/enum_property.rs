use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct EnumProperty {
    pub enum_unk: [u8; 9],
}
