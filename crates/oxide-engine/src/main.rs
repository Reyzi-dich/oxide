mod engine;
mod runtime;
mod journal;

use std::path::PathBuf;

use clap::Parser;

use engine::{ Engine, EngineOptions };

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

  let mut engine = Engine::new(args.name);
  
  if let Err(error) = engine.run() {
    eprintln!("{error}");

    std::process::exit(1);
  }

}
