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
  read_file(&config.filename)
}

fn read_file(file_path: &str) -> Result<(), Box<Error>> {
  let mut f = File::open(file_path)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  println!("File content:\n{}", contents);
  Ok(())
}
