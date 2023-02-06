use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
  /// Path to root of project to perform analysis on
  #[arg(short, long)]
  path: PathBuf,
}
