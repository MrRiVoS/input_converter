use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
  input: PathBuf,
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
    lines: Vec<String>,
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

fn is_commentary_line(line: &str) -> bool {
    line.trim_start().starts_with("**")
}

fn is_empty_line(line: &str) -> bool {
    line.trim().is_empty()
}

fn main() {

    // Parse command line arguments
    let args = Args::parse();

    // Read the input file as big string
    let input_text = fs::read_to_string(&args.input).expect("Could not read input file");

    let mut blocks: Vec<InputBlock> = Vec::new();
    let mut current_block: Option<InputBlock> = None;

    for line in input_text.lines() {
        if let Some(block) = block_from_line(line) {
            if let Some(current) = current_block.take() {
                blocks.push(current);
            }
            current_block = Some(InputBlock { block, lines: Vec::new() });
        }

        if let Some(ref mut current) = current_block {
            if is_commentary_line(line) {
                continue;
            }
            if !is_empty_line(line) {
                current.lines.push(line.trim().to_string());
            }
        }
    }

    if let Some(last_block) = current_block {
        blocks.push(last_block);
    }

    for block in blocks {
        println!("--> Block: {:?},\n Lines: {:?}\n", block.block, block.lines);
    }
        

    // let mut current_block = None;

    // for (line_number, line) in input_text.lines().enumerate() {
    //     if let Some(block) = block_from_line(line) {
    //         println!("New block {:?} in line {}", block, line_number + 1);
    //         current_block = Some(block);
    //     }

    //     println!("{:?}: {}", current_block, line);
    // }

}
