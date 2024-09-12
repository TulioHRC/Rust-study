fn main() {
    println!("Type a number: ");
    let mut num = String::new();

    std::io::stdin()
        .read_line(&mut num)
        .expect("Could not read the line!");

    let mut num: u64 = num.trim().parse()
        .expect("Could not convert to u64 number.");

    'main_loop: loop {
        if (num % 2) == 1 {
            println!("{num}");
            num-=1;
        }
        num-=1;

        if num < 10 { break 'main_loop; } 
    }
}
