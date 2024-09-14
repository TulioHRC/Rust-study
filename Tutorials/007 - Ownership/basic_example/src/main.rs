fn find_u(s: &String) -> i64 {
    for (i, c) in s.char_indices() {
        if c == 'u' { return i as i64; } 
    }
    return -1;
}

fn get_last_name(s: &mut String){
    println!("\nType your last name: ");
    let mut last_name: String = String::new();

    std::io::stdin()
        .read_line(&mut last_name)
        .expect("Error while reading line.");

    last_name = last_name.trim().to_string();

    s.push_str(&format!(" {last_name}"));
}

fn main() {
    let str_literal : &str = "GOAT"; // compile time (super fast)

    println!("Type your name: ");
    let mut name: String = String::new();

    std::io::stdin()
        .read_line(&mut name)
        .expect("Error while reading line.");

    name = name.trim().to_string();
    
    let u_position = find_u(&name); // name reference (can't be changed)

    get_last_name(&mut name);

    println!("\n{name} is the {str_literal}");
    println!("\nu is located at {u_position}");
}
