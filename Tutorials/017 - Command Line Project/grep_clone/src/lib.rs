use std::{fs, error};

pub fn parse_config(args: &Vec<String>) -> Result<(&String, &String), &'static str> {
  if args.len() < 3 {
      return Err("Not enough arguments!");
  }

  let command = &args[1];
  let file_path = &args[2];

  return Ok((command, file_path));
}

// Box<dyn error::Error> means whicherver error type you want to return
pub fn run(command: &String, file_path: &String) -> Result<(), Box<dyn error::Error>>{
  let file_content = fs::read_to_string(file_path)?; // '?' returns error if needed

  println!("\n{file_content}");

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
}