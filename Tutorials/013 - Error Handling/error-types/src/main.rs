use std::io::BufRead;

fn main() -> std::io::Result<()>  {
    println!("Do you want to try an Unrecoverable Error? [y/n]");
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).expect("Failed to read line!");
    let res : char = res.trim().parse().expect("Are you dumb? - Expect I can leave a standard message for the error, unwrap no");
    if res == 'y' {
        panic!("We're panicking");
    }

    println!("Type the filename:");
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).expect("Failed to read line!");
    let res : String = res.trim().parse().expect("Failed to read line");

    let _file = match std::fs::File::open(&res) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create(res) {
                Ok(file) => file,
                Err(e) => panic!("Error when creating file! {e:?}")
            },
            other_error => {
                panic!("Internal Error: {other_error:?}");
            }
        }
    };

    println!("Now I'm using ? to error handling. Type the filename:");
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).expect("Failed to read line!");
    let res : String = res.trim().parse().expect("Failed to read line");

    let file = std::fs::File::open(res)?;

    let reader = std::io::BufReader::new(file);
    
    // Lê e imprime o conteúdo do arquivo
    for line in reader.lines() {
        println!("{}", line?);
    }

    println!("\n\n\tThank you!");

    return Ok(()); // Returns nothing
}
