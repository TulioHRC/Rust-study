use crate::garden::vegetables::Aspargus; // allows to only use the Aspargus in code

pub mod garden; // Includes garden code

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {plant:?}!");
}