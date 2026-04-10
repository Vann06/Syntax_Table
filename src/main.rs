mod grammar;
mod first_follow;

use std::collections::{HashMap, HashSet};
use std::fs;

use grammar::Grammar;
use first_follow::first_of_sequence;

fn main() {
    let input = fs::read_to_string("gramatica.txt")
        .expect("No se pudo leer el archivo");

    let grammar = Grammar::from_string(&input);

    println!("No terminales: {:?}", grammar.non_terminals);
    println!("Terminales: {:?}", grammar.terminals);

    let mut first_sets: HashMap<String, HashSet<String>> = HashMap::new();

    // Simulación manual para probar first_of_sequence
    first_sets.insert(
        "A".to_string(),
        ["a".to_string(), "ε".to_string()].into_iter().collect(),
    );

    first_sets.insert(
        "B".to_string(),
        ["b".to_string()].into_iter().collect(),
    );

    let seq = vec!["A".to_string(), "B".to_string()];
    let result = first_of_sequence(&seq, &grammar, &first_sets);

    println!("FIRST({:?}) = {:?}", seq, result);
}