const FILEPATH : &str = "../data.json";

use std::fs;
use serde_json::Value;

fn main() {
    let mut useless_vector: Vec<String> = Vec::new();

    let file = fs::File::open(FILEPATH)
        .expect("Could not read the file!");
    let json: Vec<Value> = serde_json::from_reader(file)
        .expect("File should be proper json.");

    for data in json {
        if let Some(id) = data.get("id"){
            useless_vector.push(id.to_string().replace("\"", ""));
        }
    }
    println!("{:?}", useless_vector);
}