use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struc Args {
  input: PathBuf,
}

fn main() {

  let args = Args::parse();

  let input = fs::read_to_string(&args.input)
    .expect("Could not read input file.");

  println!("{input}");
}
