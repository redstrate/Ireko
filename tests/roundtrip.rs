use binrw::{BinRead, BinWrite};
use ireko::TaggedSerialization;
use std::fs::read;
use std::io::Cursor;
use std::path::PathBuf;

#[test]
fn roundtrip_localprofile() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources");
    d.push("LocalProfile.bin");

    let mut data = read(d).unwrap();
    let mut cursor = Cursor::new(&mut data);

    let local_profile = TaggedSerialization::read_le(&mut cursor).unwrap();
    let tagged_object = &local_profile.objs[0];
    assert_eq!(tagged_object.size_in_bytes, 339);

    tagged_object.entry("SavedDataVersion").unwrap();
    tagged_object.entry("bDemoVersion").unwrap();

    // TODO: check values

    let mut new_data: Vec<u8> = Vec::new();
    {
        let mut new_cursor = Cursor::new(&mut new_data);
        local_profile.write_le(&mut new_cursor).unwrap();
    }

    // Ensure our written version is the same as retail
    assert_eq!(new_data.as_slice(), &data[..]);
}

#[test]
fn roundtrip_slot() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources");
    d.push("Slot.bin");

    let mut data = read(d).unwrap();
    let mut cursor = Cursor::new(&mut data);

    let local_profile = TaggedSerialization::read_le(&mut cursor).unwrap();
    let tagged_object = &local_profile.objs[0];
    assert_eq!(tagged_object.size_in_bytes, 900);

    tagged_object.entry("Players").unwrap();
    tagged_object.entry("Level").unwrap();

    // TODO: check values

    let mut new_data: Vec<u8> = Vec::new();
    {
        let mut new_cursor = Cursor::new(&mut new_data);
        local_profile.write_le(&mut new_cursor).unwrap();
    }

    // Ensure our written version is the same as retail
    assert_eq!(new_data.as_slice(), &data[..]);
}
