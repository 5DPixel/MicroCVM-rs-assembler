use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;

pub fn tokenize(input: &str) -> (Vec<String>, HashMap<String, usize>) {
    use std::collections::HashMap;

    let mut tokens = Vec::new();
    let mut labels = HashMap::new();
    let mut token_index = 0;

    for line in input.replace("\r", "").lines() {
        let line = line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(label) = line.strip_suffix(':') {
            labels.insert(label.to_string(), token_index);
            continue;
        }

        if line.starts_with("ds ") || line == "ds" {
            let parts = line.splitn(2, ' ').collect::<Vec<_>>();
            tokens.push("ds".to_string());
            if parts.len() > 1 {
                let rest = parts[1].trim();
                let mut in_quote = false;
                let mut current = String::new();
                for c in rest.chars() {
                    match c {
                        '"' => {
                            in_quote = !in_quote;
                            current.push(c);
                        }
                        ',' if !in_quote => {
                            if !current.trim().is_empty() {
                                tokens.push(current.trim().to_string());
                                token_index += 1;
                            }
                            current.clear();
                        }
                        _ => current.push(c),
                    }
                }
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                    token_index += 1;
                }
            }
            continue;
        }

        for token in line.replace(", ", ",").split(|c| c == ' ' || c == ',') {
            if token.is_empty() {
                continue;
            }

            tokens.push(token.to_string());

            if token == "ds" {
                continue;
            }

            token_index += 1;
        }
    }

    (tokens, labels)
}


pub fn parse_tokens(mut tokens: Vec<String>, labels: &HashMap<String, usize>) -> Vec<u16> {
    if let Some(last) = tokens.last() {
        if last != "hlt" {
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

            i += 1;
            continue;
        }

        if let Ok(op) = super::cpu::OpcodeType::try_from(token_str) {
            opcodes.push(op as u16);
        } else if let Ok(reg) = super::cpu::Register::try_from(token_str) {
            opcodes.push(reg as u16);
        } else if let Ok(imm) = token_str.parse::<u16>() {
            opcodes.push(imm);
        } else if token_str.starts_with('\'') && token_str.ends_with('\'') && token_str.len() == 3 {
            let ch = token_str.chars().nth(1).unwrap();
            opcodes.push(ch as u16);
        } else if let Ok(fun) = super::cpu::FunctionCall::try_from(token_str) {
            opcodes.push(fun as u16);
        } else if let Some(&index) = labels.get(token_str) {
            opcodes.push(index as u16);
        } else {
            panic!("Unknown token or label: {}", token_str);
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
