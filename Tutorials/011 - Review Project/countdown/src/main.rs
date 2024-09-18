use std::io;
use countdown::timer::functions;

fn main() {
    println!("Hello, world!");
    println!("\nType a number, and see how the magic works: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");

    let input: u32 = input.trim().parse().expect("Failed to convert input to number.");
    functions::countdown(input);
}
