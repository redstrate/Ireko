use crate::property::NameProperty;

use super::primary_asset_type::PrimaryAssetTypeStruct;

#[paramacro::serialized_struct("PrimaryAssetId")]
#[derive(Debug)]
pub struct PrimaryAssetIdStruct {
    #[paramacro::serialized_field = "PrimaryAssetType"]
    pub primary_asset_type: PrimaryAssetTypeStruct,

    #[paramacro::serialized_field = "PrimaryAssetName"]
    pub primary_asset_name: NameProperty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn read_id() {
        let data = [
            0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73, 0x73,
            0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72,
            0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x37, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d,
            0x61, 0x72, 0x79, 0x41, 0x73, 0x73, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00,
            0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00,
            0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x50,
            0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e, 0x65,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x41, 0x73,
            0x73, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61,
            0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x1d, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x49, 0x74, 0x65, 0x6d,
            0x5f, 0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68,
            0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x05, 0x00, 0x00, 0x00, 0x4e, 0x6f, 0x6e,
            0x65, 0x00,
        ];
        let mut cursor = Cursor::new(data);
        let decoded = PrimaryAssetIdStruct::read_le(&mut cursor).unwrap();
    }
}
