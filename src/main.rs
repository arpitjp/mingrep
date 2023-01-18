use std::{env, process};
use mingrep_arpitjp::{Config, run};

fn main() {
 let args: Vec<String> = env::args().collect();
 let config = Config::build(&args).unwrap_or_else(|err| {
  eprintln!("Error parsing arguments: {err}");
  process::exit(1);
 });
 if let Err(e) = run(config) {
  eprintln!("Application error: {e}");
  process::exit(1);
 }
}