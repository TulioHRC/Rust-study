fn fibonacci(n: u32) -> u64 {
    let sqrt5 = (5.0f64).sqrt();
    let a_n = 1.0/sqrt5 * (((1.0 + sqrt5) / 2.0).powf(n as f64) - ((1.0 - sqrt5) / 2.0).powf(n as f64));

    return a_n as u64;
}

fn main() {
    println!("nth Fibonacci number!\n");
    println!("Type your number");

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Could not read the line!");

    let n: u32 = n.trim().parse()
        .expect("Type a unsigned int 32 bits, please.");

    let n_fib = fibonacci(n);

    println!("{n}th = {n_fib}");

}
