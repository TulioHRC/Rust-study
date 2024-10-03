use std::{env, process};
use grep_clone::{parse_config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = match parse_config(args.into_iter()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem parsing arguments: {e}");
            process::exit(1);
        }
    };

    let ignore_case: bool = std::env::var("IGNORE_CASE").is_ok();

    if let Err(error) = run(&query, &file_path, ignore_case) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}

