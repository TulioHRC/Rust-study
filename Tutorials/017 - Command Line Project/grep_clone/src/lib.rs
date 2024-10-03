use std::{fs, error};

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<(String, String), &'static str> {
  args.next(); // First argument is the local path


  let query = match args.next() {
    Some(arg) => arg,
    None => return Err("Didn't get query argument!")
  };
  let file_path = match args.next() {
    Some(arg) => arg,
    None => return Err("Didn't get file path argument!")
  };

  return Ok((query, file_path));
}

// lifetime of the full_text is the same that its result in the query
pub fn search_lines_with<'a>(query: &String, full_text: &'a String, ignore_case: bool) -> Vec<String>{
  let query = if ignore_case == false { query } else { &query.to_lowercase() };

  return full_text
    .lines()
    .filter(|line| {
      let line_to_analysis = if ignore_case == false { line.to_string() } else { line.to_string().to_lowercase() };
      return line_to_analysis.contains(query);
    })
    .map(|x| x.to_string())
    .collect();
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
  fn parse_config_no_arguments(){
    let args: Vec<String> = vec!["Test".to_string()];
    assert_eq!(parse_config(args.into_iter()), Err("Didn't get query argument!"));
  }

  #[test]
  fn parse_config_one_argument(){
    let args: Vec<String> = vec!["Test".to_string(), "query".to_string()];
    assert_eq!(parse_config(args.into_iter()), Err("Didn't get file path argument!"));
  }

  #[test]
  fn parse_config_right_arguments(){
    let args: Vec<String> = vec![String::from("binary_path"), String::from("test"), String::from("test.txt")];
    assert_eq!(parse_config(args.into_iter()), Ok(("test".to_string(), "test.txt".to_string())));
  }

  #[test]
  fn parse_config_more_arguments(){
    let args: Vec<String> = vec![String::from("binary_path"), String::from("test"), String::from("test.txt"), String::from("additional_argument")];
    assert_eq!(parse_config(args.into_iter()), Ok(("test".to_string(), "test.txt".to_string())));
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