fn main() {
    println!("Hello, world!");

    println!("My name is {}, nice to meet you {}! I'm {} years old.", "Tulio", "Rihanna", 9);
    println!("My name is {0}, nice to meet you {0}!", "Tulio"); // Place holders
    let name = "Tulio";
    println!("My name is {name}"); // with arguments
    println!("My name is {name2}", name2="Joao"); 
    println!("{:?}", ("Tulio", 2, true)); // print structures, (like it's written in code)
    println!("{0} Binary: {:b} is cool {}.", 4, "Josh");
}
