use std::{error::Error, fs, env};

pub struct Config {
  query: String,
  file_path: String,
  ignore_case: bool
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Ok(Config {query, file_path, ignore_case})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.file_path)?;
  let matches = if config.ignore_case {
    search_insensitive(&config.query, &content)
  } else {
    search(&config.query, &content)
  };
  for m in matches {
    println!("{}\t{}", m.no, m.line_content);
  }
  Ok(())
}

pub struct Matches<'a>{
  no: usize,
  line_content: &'a str
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<Matches<'a>> {
  let mut result = Vec::new();
  for (no, line_content) in content.lines().enumerate() {
    if line_content.contains(&query) {
      result.push(Matches{no: no+1, line_content});
    }
  }
  result
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<Matches<'a>> {
  let mut result = Vec::new();
  for (no, line_content) in content.lines().enumerate() {
    if line_content.to_lowercase().contains(&query.to_lowercase()) {
      result.push(Matches{no: no+1, line_content});
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
    let matches = search(query, content);
    assert_eq!(matches[0].line_content, "safe, fast, productive.");
  }
}