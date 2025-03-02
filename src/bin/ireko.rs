use binrw::BinRead;
use ireko::CompressedSaveFile;
use ireko::save_object::generic::GenericTaggedObject;
use std::env;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut data = Cursor::new(std::fs::read(&args[1])?);

    let compressed = CompressedSaveFile::<GenericTaggedObject>::read_le(&mut data)?;
    println!("{:#?}", compressed);

    // useful for creating new structed objs
    /*for object in &compressed.value.objs {
        println!("NEW OBJ");
        for entry in &object.entries {
            println!("#[paramacro::serialized_field = {:#?}]", entry.name);
            println!("unknown: {}", entry.type_name);
            println!("");
        }
    }*/

    Ok(())
}
