use std::fs::File;
use std::io;
use std::io::prelude::*;

use rand::distributions::Standard;
use rand::prelude::*;

use bb2::Move;

fn write_string_to_file(string: &str, file_path: &str) -> io::Result<()> {
    let mut writer = File::create(file_path)?;
    writer.write_all(string.as_bytes())?;
    Ok(())
}

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut reader = File::open(file_path)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    Ok(data)
}

fn main() {
    let file_path = String::from("/tmp/move.json");
    let a: Move = StdRng::from_entropy().sample(Standard);

    // Serialize the Move a to file as JSON
    let serialized = serde_json::to_string(&a).unwrap();
    write_string_to_file(&serialized, &file_path).expect("Error while writing the file");

    // Deserialize from file to the Move b
    let json = read_file_to_string(&file_path).expect("Error while reading the file");
    let b: Move = serde_json::from_str(&json).unwrap();

    println!("Move struct: {:?}", a);
    println!("Deserialized JSON as string: {:?}", b);
}
