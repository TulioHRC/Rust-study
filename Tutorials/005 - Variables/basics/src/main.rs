const CONSTVAR : u8 = 5; // Defined in compiling time (needs type)

// YOU CANNOT DECLARE VARIABLE WITHOUT VALUE

fn main() {
    let x = 10; // Immutable variable - default i32
    let mut y = 0; // Mutable variable

    println!("CONSTVAR = {CONSTVAR}"); // 5
    println!("x = {x}"); // 10
    println!("y = {y}"); // 0

    //x = 1; will cause error
    y = 2;

    println!("y = {y}"); // 2

    // y = true; will cause error, you can't change the type of a mut variable

    // Shadowing
    let x = true; // you re-declare x, can change its type - bool
    println!("x = {x}"); // true
}
