mod common;

/// Properties
pub mod property;

/// Top-level save objects
pub mod save_object;

/// Various structures
pub mod structure;

use binrw::helpers::until_eof;
use std::fs::write;
use std::io::{Cursor, Read};

use binrw::{BinRead, BinResult, BinWrite, binrw};
use flate2::bufread::ZlibDecoder;

#[binrw]
#[derive(Debug)]
pub struct TaggedSerialization<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + 'a,
    for<'a> T: BinWrite<Args<'a> = ()> + 'a,
{
    pub size_in_bytes: u32,
    pub objs: T,
}

#[binrw::parser(reader, endian)]
fn read_compressed_data(compressed_size: u64, uncompressed_size: u64) -> BinResult<Vec<u8>> {
    let mut compressed_data = vec![0; compressed_size as usize];
    reader.read_exact(&mut compressed_data)?;

    let mut uncompressed = vec![0; uncompressed_size as usize];

    let mut d = ZlibDecoder::new(&*compressed_data);
    let size = d.read(&mut uncompressed)?;

    Ok(uncompressed[..size].to_vec())
}

// TODO: there's no point in using a parser, we should just use map()
#[binrw::parser(reader, endian)]
fn read_tagged_data<T>(data_blocks: &Vec<CompressedBlock>) -> BinResult<TaggedSerialization<T>>
where
    for<'a> T: BinRead<Args<'a> = ()> + 'a,
    for<'a> T: BinWrite<Args<'a> = ()> + 'a,
{
    let data_vecs: Vec<Vec<u8>> = data_blocks.iter().map(|x| x.data.clone()).collect();
    let combined_data = data_vecs.concat();
    write("output.bin", &combined_data);

    let mut cursor = Cursor::new(&combined_data);

    TaggedSerialization::<T>::read_le(&mut cursor)
}

#[binrw]
#[brw(magic = 0x9e2a83c1u32)]
#[derive(Debug)]
pub struct CompressedBlock {
    #[br(pad_before = 6)]
    #[br(pad_after = 5)]
    pub unk: u8, // always 2
    pub a_compresed_size: u64,
    pub a_uncompresed_size: u64,
    pub b_compresed_size: u64,
    pub b_uncompresed_size: u64,
    #[br(parse_with = read_compressed_data, args(a_compresed_size, a_uncompresed_size))]
    pub data: Vec<u8>,
}

#[binrw]
#[derive(Debug)]
pub struct CompressedSaveFile<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + 'a,
    for<'a> T: BinWrite<Args<'a> = ()> + 'a,
{
    #[br(parse_with = until_eof)]
    #[br(temp)]
    #[bw(ignore)]
    pub data: Vec<CompressedBlock>,
    #[br(parse_with = read_tagged_data, args(&data))]
    pub value: TaggedSerialization<T>,
}
