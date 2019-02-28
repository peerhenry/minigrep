use minigrep::Config;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    panic!("incorrect");
  }
  let config: Config = Config::new(args);
  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);
  read_file(&config.filename);
}

fn read_file(file_path: &str) {
  let mut f = File::open(file_path).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("Something went wrong while reading the file");
  println!("File content:\n{}", contents);
}
