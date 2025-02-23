use binrw::BinRead;
use flate2::Decompress;
use flate2::bufread::ZlibDecoder;
use ireko::{CompressedSaveFile, TaggedSerialization};
use std::env;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut data = Cursor::new(std::fs::read(&args[1])?);

    let compressed = CompressedSaveFile::read_le(&mut data)?;

    let mut output = vec![];

    let mut uncompressed = vec![0; compressed.a_uncompresed_size as usize];

    let decompress = Decompress::new(true);

    let mut d = ZlibDecoder::new_with_decompress(&*compressed.compressed_data, decompress);
    let size = d.read(&mut uncompressed)?;
    output.extend_from_slice(&uncompressed[..size]);
    std::fs::write("output.bin", &output)?;

    let mut cursor = Cursor::new(&uncompressed);

    let save_data = TaggedSerialization::read_le(&mut cursor);
    println!("{:#?}", save_data);

    Ok(())
}
