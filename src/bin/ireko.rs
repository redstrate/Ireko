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
    println!("{:#?}", compressed);

    Ok(())
}
