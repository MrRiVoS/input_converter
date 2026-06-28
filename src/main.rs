use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
  input: PathBuf,
}

#[derive(Debug)]
enum Keyword {
    Grid,
    Di,
    Dj,
    Dk,
    Unknown(String),
}

fn classify_keyword(line :&str) -> Option<Keyword> {
    let word = line.trim().split_whitespace().next()?;

    // remove the '*' if it exists
    let word = word.strip_prefix('*').unwrap_or(word);

    match word.to_uppercase().as_str() {
        "GRID" => Some(Keyword::Grid),
        "DI"   => Some(Keyword::Di),
        "DJ"   => Some(Keyword::Dj),
        "DK"   => Some(Keyword::Dk),
        other  => Some(Keyword::Unknown(other.to_string())),
    }
}

fn main() {

    let args = Args::parse();

    let input_bytes = fs::read(&args.input)
      .expect("Could not read input file.");

    let input_text: String = input_bytes.into_iter().map(|b| b as char).collect();

    // println!("File size: {} bytes", input.len());
    // println!("{input_text}");
    for (i, line) in input_text.lines().enumerate() {
        if let Some(keyword) = classify_keyword(line) {
            println!("{:04}: {:?} -> {}", i + 1, keyword, line);
        }
    }

}
