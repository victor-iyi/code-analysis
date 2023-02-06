use clap::Parser;
use code_analysis::cli::Cli;

fn main() {
  let args = Cli::parse();
  println!("{args:?}");
}
