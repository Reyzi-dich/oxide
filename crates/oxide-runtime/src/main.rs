mod runtime;
mod journal;

use std::path::PathBuf;

use clap::Parser;

use runtime::{ Runtime, RuntimeOptions };

#[derive(Parser)]
struct Args {
  name:    PathBuf,

  #[arg(short, long)]
  trace: bool,

  #[arg(short, long)]
  debug: bool
}

fn main() {
  let args = Args::parse();

  let mut runtime = Runtime::new(args.name);
  
  if let Err(error) = runtime.run() {
    eprintln!("{error}");

    std::process::exit(1);
  }

}
