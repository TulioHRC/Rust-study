fn main() {
    println!("Fahrenheit to Celsius converter.\n");
    println!("Fahrenheit temperature: ");

    let mut fahrenheit = String::new();
    std::io::stdin()
        .read_line(&mut fahrenheit)
        .expect("\nFailed to read the input!");

    let fahrenheit : f32 = fahrenheit.trim().parse()
        .expect("Please type a valid float number for 32 bits.");
    
    let celsius : f32 = (5.0 / 9.0) * (fahrenheit - 32.0);
    println!("{fahrenheit}ºF = {celsius}ºC");
}
