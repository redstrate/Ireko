use crate::Property;
use binrw::binrw;
use crate::common::read_string_with_length;
use crate::guid::Guid;

#[binrw]
#[derive(Debug)]
pub struct DateTimeStruct {
    pub unk: [u8; 25],
}

#[binrw]
#[derive(Debug)]
pub struct DALoadOptionStruct {
    pub unk: [u8; 69], // Contains LoadType property
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
pub struct PrimaryAssetNameProperty {
    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub property_name: String,

    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub type_name: String,

    #[br(if(property_name != "None"))]
    #[br(args { magic: &type_name})]
    pub key: Option<Box<Property>>,
}

#[binrw]
#[derive(Debug)]
pub struct CarryCountProperty {
    #[br(parse_with = read_string_with_length)]
    #[bw(ignore)]
    pub property_name: String,

    #[br(args { magic: &property_name})]
    pub key: Option<Box<Property>>,
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
    #[br(pad_before = 17)]
    pub module_level: PrimaryAssetNameProperty,
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
    #[br(pad_before = 17)]
    pub guid: Guid
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
