use std::env;
use std::fs::read_to_string;
use std::io::{self};

mod assemble;
mod cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: mca <input_file> -o <output_file>");
        std::process::exit(1);
    }
    if args[2] != "-o" {
        eprintln!("Expected -o flag before output file.");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[3];

    let input = read_to_string(input_file)?;

    let tokens = assemble::tokenize(&input);
    let opcodes = assemble::parse_tokens(tokens);
    assemble::create_binary(opcodes, output_file)?;

    println!("Assembly successfully compiled to: {}", output_file);
    Ok(())
}
