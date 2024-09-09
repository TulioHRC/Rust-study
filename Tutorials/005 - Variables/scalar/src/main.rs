fn main() {
    // intenger
    let x = 5; // default is i32 (u unsigned - i signed)
    let x: u8 = 1;
    let x: u16 = 0xff; // hex
    println!("{x}"); // 255
    let x: u32 = 0b1111_0000; // binary
    println!("{x}"); // 240
    let x: u64 = 1_000; // _ division (only visual)
    println!("{x}"); // 1000
    let x: u128 = 1;
    let x: usize = 1; // depends into your OS (32 or 64)

    // float
    let y = 0.5; // default is f64
    let y: f32 = 0.5;
    let y: f64 = 0.5;

    // bool
    let c: bool = true; // 8bits type (just for true or false)

    // char
    let d: char = 'é'; 
}
