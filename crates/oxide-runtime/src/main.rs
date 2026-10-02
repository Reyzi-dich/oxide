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

fn main() -> miette::Result<()> {
  let args = Args::parse();

  let mut runtime = Runtime::new(args.name);
  
  runtime.run()?;

  Ok(())
}
