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
const BOLD: &str = "\x1b[1m";
const RESET_BOLD: &str = "\x1b[22m";

fn print_help() {
    println!(
        "{}{}MicroCVM Assembler (mca){}{}\n\n\
         {}{}Usage:{}{}\n\
         \t{}mca{} <{}input.asm{}> {}-o{} <{}output.bin{}>\n\n\
         {}Flags:{}\n\
         \t{}-o{}       Specify output binary file name\n\
         \t{}-h{}       Show this help message\n\
         \t{}--version{} Show current version\n",
        CYAN,
        BOLD,
        RESET_BOLD,
        RESET,
        YELLOW,
        BOLD,
        RESET,
        RESET_BOLD,
        BOLD,
        RESET_BOLD,
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

    if args.contains(&"--version".to_string()) {
        println!(
            "{}{}MicroCVM assembler (mca){}{}\n\n
            Created by {}{}Jude Routledge{}{}\n
            Version {}{}{}{}{}",
            BOLD,
            CYAN,
            RESET_BOLD,
            RESET,
            BOLD,
            GREEN,
            RESET_BOLD,
            RESET,
            BOLD,
            CYAN,
            env!("CARGO_PKG_VERSION"),
            RESET,
            RESET_BOLD
        );
    }

    if args.len() != 4 {
        eprintln!(
            "{}{}fatal error:{}{} mca (no input files detected, use {}{}-h{}{} for help)",
            BOLD, RED, RESET, RESET_BOLD, BOLD, CYAN, RESET, RESET_BOLD
        );
        std::process::exit(1);
    }

    if args[2] != "-o" {
        eprintln!(
            "{}{}fatal error:{}{} mca (no input files detected, use {}{}-h{}{} for help)",
            BOLD, RED, RESET, RESET_BOLD, BOLD, CYAN, RESET, RESET_BOLD
        );
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[3];

    if let Err(e) = metadata(input_file) {
        eprintln!(
            "{}{}error:{}{} could not access input file '{}'. Error: {}",
            BOLD, RED, RESET, RESET_BOLD, input_file, e
        );
        std::process::exit(1);
    }

    if let Ok(_) = metadata(output_file) {
        print!(
            "{}{}warning:{}{} output file '{}' already exists. overwrite? (y/n): ",
            BOLD, YELLOW, RESET, RESET_BOLD, output_file
        );
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("{}operation aborted. exiting...{}", RED, RESET);
            std::process::exit(0);
        }
    }

    let input = read_to_string(input_file)?;

    let tokens = assemble::tokenize(&input);
    let opcodes = assemble::parse_tokens(tokens);
    assemble::create_binary(opcodes, output_file)?;

    println!(
        "{}{}success:{}{} assembly compiled to {}{}{}{}{}",
        BOLD, GREEN, RESET, RESET_BOLD, CYAN, BOLD, output_file, RESET, RESET_BOLD
    );
    Ok(())
}
