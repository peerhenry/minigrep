use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config: Config = parse_config(&args);
  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);
  read_file(&config.filename);
}

struct Config {
  query: String,
  filename: String
}

fn parse_config(args: &[String]) -> Config {
  Config{
    query: args[1].clone(),
    filename: args[2].clone()
  }
}

fn read_file(file_path: &str) {
  let mut f = File::open(file_path).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("Something went wrong while reading the file");
  println!("File content:\n{}", contents);
}
