use binrw::BinRead;
use ireko::CompressedSaveFile;
use std::env;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut data = Cursor::new(std::fs::read(&args[1])?);

    let compressed = CompressedSaveFile::read_le(&mut data)?;
    println!("{:#?}", compressed);

    Ok(())
}
