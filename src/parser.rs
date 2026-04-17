use std::fs;
use std::io;

use crate::grammar::Grammar;
use crate::item::Item;

pub fn load_grammar_from_file(path: &str) -> io::Result<Grammar> {
    let content = fs::read_to_string(path)?;

    Grammar::try_from_string(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn parse_item(input: &str) -> io::Result<Item> {
    let parts: Vec<&str> = input.split("->").collect();

    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Ítem inválido. Use formato: A -> . B c",
        ));
    }

    let lhs = parts[0].trim().to_string();

    let tokens: Vec<String> = parts[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let dot_pos = tokens.iter().position(|t| t == ".").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "El ítem debe contener un punto '.'",
        )
    })?;

    let dot_count = tokens.iter().filter(|t| *t == ".").count();
    if dot_count != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "El ítem debe contener exactamente un punto '.'",
        ));
    }

    let mut rhs = tokens.clone();
    rhs.remove(dot_pos);

    Ok(Item { lhs, rhs, dot: dot_pos })
}