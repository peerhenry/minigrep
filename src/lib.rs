use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: Vec<String>) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("not enough parameters");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let text = read_file(&config.filename)?;
  let matching_lines = if config.case_sensitive {
    search(&config.query, &text)
  } else {
    search_case_insensitive(&config.query, &text)
  };
  for line in matching_lines {
    println!("{}", line);
  }
  Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<Error>> {
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  Ok(contents)
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
  let mut output: Vec<&str> = vec![];
  for line in text.lines() {
    if line.contains(query) {
      output.push(line)
    }
  }
  output
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut output: Vec<&str> = vec![];
  for line in text.lines() {
    if line.to_lowercase().contains(&query) {
      output.push(line)
    }
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_works() {
    // arrange
    let query = "duct";
    let text = "Rust:\nsafe, fast, productive.\nPick three.";
    let expected = vec!["safe, fast, productive."];
    // act
    let result = search(query, text);
    // assert
    assert_eq!(expected, result);
  }

  #[test]
  fn case_sensitive() {
    // arrange
    let text = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
    let query = "duct";
    let expected = vec!["safe, fast, productive."];
    // act
    let result = search(query, text);
    // assert
    assert_eq!(expected, result);
  }

    #[test]
  fn case_insensitive() {
    // arrange
    let text = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
    let query = "rUsT";
    let expected = vec!["Rust:", "Trust me."];
    // act
    let result = search_case_insensitive(query, text);
    // assert
    assert_eq!(expected, result);
  }
}