use binrw::{BinRead, BinReaderExt, BinResult};

pub(crate) fn read_bool_from<T: From<u8> + PartialEq>(x: T) -> bool {
    x == T::from(1u8)
}

#[binrw::parser(reader, endian)]
pub(crate) fn read_string_with_length() -> BinResult<String> {
    // last byte is the null terminator which Rust ignores
    let length = u32::read_le(reader)? as usize - 1;
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
