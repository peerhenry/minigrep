use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(args: Vec<String>) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("not enough parameters");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config {query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let text = read_file(&config.filename)?;
  search(&config.query, &text);
  Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<Error>> {
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  println!("File content:\n{}", contents);
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dummy() {
    // arrange
    let query = "duct";
    let text = "Rust:\nsafe, fast, productive.\nPick three.";
    let expected = vec!["safe, fast, productive."];
    // act
    let result = search(query, text);
    // assert
    assert_eq!(expected, result);
  }
}