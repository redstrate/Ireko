use binrw::BinResult;
use binrw::{BinRead, BinWrite};

pub(crate) fn read_bool_from<T: From<u8> + PartialEq>(x: T) -> bool {
    x == T::from(1u8)
}

pub(crate) fn write_bool_as<T: From<u8>>(x: &bool) -> T {
    if *x { T::from(1u8) } else { T::from(0u8) }
}

#[binrw::parser(reader, endian)]
pub(crate) fn read_string_with_length() -> BinResult<String> {
    let length = u32::read_le(reader)? as usize;
    if length == 0 {
        return Ok(String::default());
    }
    // last byte is the null terminator which Rust ignores
    let length = length - 1;
    let mut bytes: Vec<u8> = vec![0u8; length];
    // TODO: there was to be way to read this all in one go?
    for i in 0..length {
        bytes[i] = u8::read_le(reader)?;
    }
    u8::read_le(reader)?; // read null terminator
    String::from_utf8(bytes).or(Err(binrw::Error::AssertFail {
        pos: 0,
        message: "dummy".to_string(),
    }))
}

#[binrw::writer(writer, endian)]
pub(crate) fn write_string_with_length(string: &String) -> BinResult<()> {
    if string.is_empty() {
        let length = 0u32;
        length.write_le(writer)?;
        return Ok(());
    }
    // + 1 for the null terminator
    let length = string.len() as u32 + 1;
    length.write_le(writer)?;
    for char in string.chars() {
        let byte = char as u8;
        byte.write_le(writer)?;
    }
    let null_terminator = 0u8;
    null_terminator.write_le(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, binread};
    use std::io::Cursor;
    use std::string::String;

    #[test]
    fn read_bool_u8() {
        assert!(!read_bool_from::<u8>(0u8));
        assert!(read_bool_from::<u8>(1u8));
    }

    #[test]
    fn read_string() {
        // From LocalProfile.sav i think
        let data = [
            0x0a, 0x00, 0x00, 0x00, 0x72, 0x65, 0x64, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65, 0x00,
        ];

        #[binread]
        struct TestStruct {
            #[br(parse_with = read_string_with_length)]
            value: String,
        }

        let mut cursor = Cursor::new(data);
        let decoded = TestStruct::read_le(&mut cursor).unwrap();
        assert_eq!(decoded.value, "redstrate");
    }
}
