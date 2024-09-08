use rand::Rng; // defines the randomizing method
use std::cmp::Ordering;

fn main() {
    println!("My guessing game!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 1..=100 start..=final

    loop { // like a while(true)
        let mut guess = String::new();
        // let is used to declare a variable, and mut to mutable, because immutable is the default
        // String::new() creates a empty string

        println!("Type your guess: ");
        
        // io - input/output lib
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        // u32 is similar to unsigned int in C
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number."); // Conversion
    
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You're cheating!");
                break;
            }
        }
    }
}
