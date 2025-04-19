use std::fs::File;
use std::io::{self, Write};

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .to_lowercase()
        .replace(", ", ",")
        .replace("\r", "")
        .lines()
        .map(|line| line.split(';').next().unwrap_or(""))
        .flat_map(|line| {
            line.split(|c| c == ' ' || c == ',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .collect()
}

pub fn parse_tokens(mut tokens: Vec<String>) -> Vec<u16> {
    if let Some(last) = tokens.last() {
        if last.to_lowercase() != "hlt" {
            tokens.push("hlt".to_string());
        }
    }

    let mut opcodes: Vec<u16> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let token_str = tokens[i].as_str();

        if token_str == "ds" {
            i += 1;
            if i >= tokens.len() {
                break;
            }

            let string = tokens[i].trim_matches('"');
            let mut bytes = string.bytes().collect::<Vec<_>>();
            if bytes.len() % 2 != 0 {
                bytes.push(0);
            }

            for chunk in bytes.chunks(2) {
                let lo = chunk[0];
                let hi = chunk[1];
                let value = (hi as u16) << 8 | (lo as u16);
                opcodes.push(value);
            }
        } else if let Ok(op) = super::cpu::OpcodeType::try_from(token_str) {
            opcodes.push(op as u16);
        } else if let Ok(reg) = super::cpu::Register::try_from(token_str) {
            opcodes.push(reg as u16);
        } else if let Ok(imm) = token_str.parse::<u16>() {
            opcodes.push(imm);
        } else if let Ok(fun) = super::cpu::FunctionCall::try_from(token_str) {
            opcodes.push(fun as u16);
        }

        i += 1;
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
