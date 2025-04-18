use std::env;
use std::fs::{metadata, read_to_string};
use std::io::{self, Write};

mod assemble;
mod cpu;

// ANSI color codes
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

fn print_help() {
    println!(
        "{}MicroCVM Assembler (mca){}\n\n\
         {}Usage:{}\n\
         \tmca <{}input.asm{}> {}-o{} <{}output.bin{}>\n\n\
         {}Flags:{}\n\
         \t{}-o{}       Specify output binary file name\n\
         \t{}-h{}       Show this help message\n",
        CYAN,
        RESET,
        YELLOW,
        RESET,
        GREEN,
        RESET,
        CYAN,
        RESET,
        GREEN,
        RESET,
        YELLOW,
        RESET,
        CYAN,
        RESET,
        CYAN,
        RESET
    );
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        return Ok(());
    }

    if args.len() != 4 {
        eprintln!(
            "{}fatal error:{} mca (no input files detected, use {}-h{} for help)",
            RED, RESET, CYAN, RESET
        );
        std::process::exit(1);
    }

    if args[2] != "-o" {
        eprintln!(
            "{}error:{} expected {}-o{} flag before output file.",
            RED, RESET, CYAN, RESET
        );
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[3];

    if let Err(e) = metadata(input_file) {
        eprintln!(
            "{}error:{} could not access input file '{}'. Error: {}",
            RED, RESET, input_file, e
        );
        std::process::exit(1);
    }

    if let Ok(_) = metadata(output_file) {
        print!(
            "{}warning:{} output file '{}' already exists. overwrite? (y/n): ",
            YELLOW, RESET, output_file
        );
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("{}operation aborted. Exiting...{}", RED, RESET);
            std::process::exit(0);
        }
    }

    let input = read_to_string(input_file)?;

    let tokens = assemble::tokenize(&input);
    let opcodes = assemble::parse_tokens(tokens);
    assemble::create_binary(opcodes, output_file)?;

    println!(
        "{}success:{} assembly compiled to {}{}{}",
        GREEN, RESET, CYAN, output_file, RESET
    );
    Ok(())
}
