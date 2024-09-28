use std::{env, process};
use grep_clone::{parse_config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = match parse_config(&args) {
        Ok(config) => config,
        Err(e) => {
            println!("Problem parsing arguments: {e}");
            process::exit(1);
        }
    };

    if let Err(error) = run(query, file_path) {
        println!("Application error: {error}");
        process::exit(1);
    }
}

