pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(args: Vec<String>) -> Self {
    Config {
      query: args[1].clone(),
      filename: args[2].clone()
    }
  }
}