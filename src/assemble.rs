use std::fs::File;
use std::io::{self, Write};

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .to_lowercase()
        .replace(", ", ",")
        .replace("\r", "")
        .split('\n')
        .flat_map(|line| {
            line.split(|c| c == ' ' || c == ',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .collect()
}

pub fn parse_tokens(tokens: Vec<String>) -> Vec<u16> {
    let mut opcodes: Vec<u16> = Vec::new();

    for token in tokens.iter() {
        let token_str = token.as_str();

        if let Ok(op) = super::cpu::OpcodeType::try_from(token_str) {
            opcodes.push(op as u16);
        } else if let Ok(reg) = super::cpu::Register::try_from(token_str) {
            opcodes.push(reg as u16);
        } else if let Ok(imm) = token_str.parse::<u16>() {
            opcodes.push(imm);
        } else if let Ok(fun) = super::cpu::FunctionCall::try_from(token_str) {
            opcodes.push(fun as u16);
        }
    }

    opcodes
}

pub fn create_binary(tokens: Vec<u16>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for token in tokens {
        let bytes = token.to_le_bytes();
        file.write_all(&bytes)?;
    }

    Ok(())
}
