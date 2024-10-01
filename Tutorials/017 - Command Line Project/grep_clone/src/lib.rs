use std::{fs, error};

pub fn parse_config(args: &Vec<String>) -> Result<(&String, &String), &'static str> {
  if args.len() < 3 {
      return Err("Not enough arguments!");
  }

  let query = &args[1];
  let file_path = &args[2];

  return Ok((query, file_path));
}

// lifetime of the full_text is the same that its result in the query
pub fn search_lines_with<'a>(query: &String, full_text: &'a String, ignore_case: bool) -> Vec<String>{
  let mut found_lines = Vec::new();
  let query = if ignore_case == false { query } else { &query.to_lowercase() };

  for line in full_text.lines() {
    let line_to_analysis = if ignore_case == false { line } else { &line.to_lowercase() };
    if line_to_analysis.to_lowercase().contains(query) {
      found_lines.push(line.to_string());
    }
  }

  return found_lines;
}

// Box<dyn error::Error> means whicherver error type you want to return
pub fn run(query: &String, file_path: &String, ignore_case: bool) -> Result<(), Box<dyn error::Error>>{
  let file_content = fs::read_to_string(file_path)?; // '?' returns error if needed

  for line in search_lines_with(query, &file_content, ignore_case){
    println!("{line}");
  }

  return Ok(());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_config_less_arguments(){
    let args: Vec<String> = vec!["Test".to_string()];
    assert_eq!(parse_config(&args), Err("Not enough arguments!"));
  }

  #[test]
  fn parse_config_right_arguments(){
    let args: Vec<String> = vec![String::from("binary_path"), String::from("test"), String::from("test.txt")];
    assert_eq!(parse_config(&args), Ok((&"test".to_string(), &"test.txt".to_string())));
  }

  #[test]
  fn parse_config_more_arguments(){
    let args: Vec<String> = vec![String::from("binary_path"), String::from("test"), String::from("test.txt"), String::from("additional_argument")];
    assert_eq!(parse_config(&args), Ok((&"test".to_string(), &"test.txt".to_string())));
  }

  #[test]
  fn search_lines_with_one_result(){
    let query = String::from("duct");
    let file_content = String::from("\
Rust:
safe, fast, productive
Pick three.");
    
    assert_eq!(vec!["safe, fast, productive"], search_lines_with(&query, &file_content, false));
  }

  #[test]
  fn search_lines_with_no_result(){
    let query = String::from("duct");
    let file_content = String::from("\
Rust:
safe, fast
Pick three.");
    
    assert_eq!(Vec::<String>::new(), search_lines_with(&query, &file_content, false));
  }

  #[test]
  fn search_lines_with_multiple_result(){
    let query = String::from("duct");
    let file_content = String::from("\
Rust productiveness:
safe, fast, productive
Pick three ducks.");
    
    assert_eq!(vec!["Rust productiveness:", "safe, fast, productive"], search_lines_with(&query, &file_content, false));
  }

  #[test]
  fn search_lines_with_case_sensitive(){
    let query = String::from("RusT");
    let file_content = String::from("\
Rust:
just
trust me
I love you!");

    assert_eq!(vec!["Rust:", "trust me"], search_lines_with(&query, &file_content, true));
  }
}