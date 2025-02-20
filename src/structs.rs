use crate::{Property, StringBasedProperty};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    pub unk: [u8; 25],
}

#[binrw]
#[derive(Debug)]
pub struct DALoadOptionStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct SaveSlotInfoStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct DACharacterCommonStatusStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct ParamsStruct {
    pub params_value_type_length: u32,

    pub params_value: Box<Property>,
}

#[binrw]
#[derive(Debug)]
pub struct PrimaryAssetNameProperty {
    #[br(temp)]
    #[bw(ignore)]
    pub property_name_length: u32,
    #[br(count = property_name_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    pub property_name: String,

    #[br(temp)]
    #[bw(ignore)]
    #[br(if(property_name != "None"))]
    pub type_length: u32,
    #[br(count = type_length)]
    #[bw(map = |x : &String | x.as_bytes())]
    #[br(map = | x: Vec<u8> | String::from_utf8(x).unwrap().trim_matches(char::from(0)).to_string())]
    #[br(if(property_name != "None"))]
    pub type_name: String,

    #[br(if(property_name != "None"))]
    #[br(args { magic: &type_name})]
    pub key: Option<Box<StringBasedProperty>>,
}

#[binrw]
#[derive(Debug)]
pub struct PrimaryAssetTypeStruct {
    #[br(pad_before = 17)]
    name: PrimaryAssetNameProperty,
    #[br(pad_before = 9)] // "None" and it's length in bytes plus the null terminator
    #[br(pad_after = 9)] // ditto
    primary_asset_name: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct PrimaryAssetIdStruct {
    #[br(pad_before = 17)]
    asset: PrimaryAssetNameProperty,
}

#[binrw]
#[derive(Debug)]
pub struct DAModuleItemDataStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct DABuildDataStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct DAAssembleIdDataStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct GuidStruct {
    pub unk: [u8; 33],
}

#[binrw]
#[derive(Debug)]
pub struct DAMachineColoringDataStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct DAModuleColorStruct {
    pub unk: [u8; 17],
}

#[binrw]
#[derive(Debug)]
pub struct LinearColorStruct {
    pub unk: [u8; 33],
}
