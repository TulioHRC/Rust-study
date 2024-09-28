use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let file_path = &args[2];

    dbg!(&args);

    println!("Reading file {file_path}:");

    let file_content = fs::read_to_string(file_path)
        .expect("Couldn't read the file.");

    println!("\n{file_content}");
}
