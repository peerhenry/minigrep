use minigrep::Config;
use std::process; // without this error is unwrap_or_else return type
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config: Config = Config::new(args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1); // why not panic?
  });
  println!("Searching for {} in file {}", config.query, config.filename);
  println!("Results:");
  println!("");
  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
  println!("");
}