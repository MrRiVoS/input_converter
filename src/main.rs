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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Grid,
    Model,
    RockFluid,
    Initial,
    Numerical,
    Run,
}

#[derive(Debug)]
struct InputBlock {
    block: Block,
    line: Vec<String>,
}

fn block_from_line(line: &str) -> Option<Block> {
    let word = line.trim().split_whitespace().next()?
        .strip_prefix('*').unwrap_or(line);

    match word.to_uppercase().as_str() {
        "GRID" => Some(Block::Grid),
        "MODEL" => Some(Block::Model),
        "ROCKFLUID" => Some(Block::RockFluid),
        "INITIAL" => Some(Block::Initial),
        "NUMERICAL" => Some(Block::Numerical),
        "RUN" => Some(Block::Run),
        _ => None,
    }
}

fn normalize_token(token: &str) -> &str {
    token.strip_prefix('*').unwrap_or(token)
}

fn classify_keyword(line :&str) -> Option<Keyword> {
    let word = line.trim().split_whitespace().next()?;

    // remove the '*' if it exists
    let word = normalize_token(word);

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

    let mut current_block = None;
    // println!("File size: {} bytes", input.len());
    // println!("{input_text}");
    for (line_number, line) in input_text.lines().enumerate() {
        // if let Some(keyword) = classify_keyword(line) {
        //     println!("{:04}: {:?} -> {}", i + 1, keyword, line);
        // }
        if let Some(block) = block_from_line(line) {
            println!("New block {:?} in line {}", block, line_number + 1);
            current_block = Some(block);
        }

        println!("{:?}: {}", current_block, line);
    }



}
