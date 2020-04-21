use std::fs::File;
use std::io;
use std::io::{Cursor, Error, ErrorKind, Read, Write};

use bson::{decode_document, encode_document};
use rand::distributions::Standard;
use rand::prelude::*;

use bb2::Move;

fn write_moves<W: Write>(writer: &mut W) -> io::Result<()> {
    for n in 0..100 {
        let mov: Move = StdRng::from_entropy().sample(Standard);
        let serialized = bson::to_bson(&mov).unwrap();
        if let bson::Bson::Document(document) = &serialized {
            encode_document(writer, document).unwrap();
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                String::from(&format!("Error while encoding to BSON the {}th Move", n)),
            ));
        }
    }
    Ok(())
}

fn read_moves<R: Read>(reader: &mut R) -> () {
    println!("###");
    while let Ok(doc) = decode_document(reader) {
        let mov: Move = bson::from_bson(bson::Bson::Document(doc))
            .expect("Error deserializing the BSON doc to a Move struct.");
        println!("{:?}", mov);
    }
}

fn main() {
    // Serialize to File
    let file_path = String::from("/tmp/move.bson");
    let mut writer = File::create(&file_path).expect("Error creating/truncating the file");
    write_moves(&mut writer).expect("Error writing to the file");
    let mut reader = File::open(&file_path).expect("Error opening the file");
    read_moves(&mut reader);

    // Serialize to Vec<u8>
    let mut writer: Vec<u8> = Vec::new();
    write_moves(&mut writer).expect("Error writing the Vec<u8>");
    let mut reader = Cursor::new(writer);
    read_moves(&mut reader);
}
