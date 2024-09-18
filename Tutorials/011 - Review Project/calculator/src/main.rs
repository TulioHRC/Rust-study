use calculator::{Operation, OperationType};
use std::{io, process};

fn main() {
    println!("Calculator!\n");

    println!("First Input: ");
    let mut input_one = String::new();
    io::stdin()
        .read_line(&mut input_one)
        .expect("Failed to read line!");
    let input_one: isize = input_one
        .trim()
        .parse()
        .expect("Failed to convert to number.");

    println!("Operation (it can be '+', '-' '*' or '/'): ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line!");
    let operation: char = operation
        .trim()
        .parse()
        .expect("Failed to convert to a char.");
    let operation_type: OperationType = match operation {
        '+' => OperationType::Sum,
        '-' => OperationType::Subtract,
        '*' => OperationType::Product,
        '/' => OperationType::Division,
        _ => {
            println!("Not available");
            process::exit(1)
        }
    };

    println!("Second Input: ");
    let mut input_two = String::new();
    io::stdin()
        .read_line(&mut input_two)
        .expect("Failed to read line!");
    let input_two: isize = input_two
        .trim()
        .parse()
        .expect("Failed to convert to number.");

    let operation: Operation = Operation::create_operation(input_one, input_two, operation_type);
    operation.display();
    operation.display_result();
}
