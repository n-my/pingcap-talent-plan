use std::io::Write;
use std::str;

use rand::distributions::Standard;
use rand::prelude::*;
use ron::ser;

use bb2::Move;

fn main() {
    let a: Move = StdRng::from_entropy().sample(Standard);

    // Serialize to Vec<u8> as JSON
    let serialized_json = serde_json::to_string(&a).expect("Error while serializing.");
    let mut json_buffer: Vec<u8> = Vec::new();
    json_buffer
        .write_all(&serialized_json.as_bytes())
        .expect("Error while writing to buffer");
    // Deserialize Vec<u8> to String
    let deserialized_json = str::from_utf8(&json_buffer).expect("Error while deserializing.");

    // Serialize to Vec<u8> as RON
    let serialized_ron = ser::to_string(&a).expect("Error while serializing.");
    let mut ron_buffer: Vec<u8> = Vec::new();
    ron_buffer
        .write_all(&serialized_ron.as_bytes())
        .expect("Error while writing to buffer");
    // Deserialize Vec<u8> to String
    let deserialized_ron = str::from_utf8(&ron_buffer).expect("Error while serializing.");

    println!("Move struct: {:?}", a);
    println!("Deserialized JSON as String: {:?}", deserialized_json);
    println!("Deserialized RON as String: {:?}", deserialized_ron);
}
