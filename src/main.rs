mod grammar;

use std::fs;
use grammar::Grammar;

fn main() {
    let input = fs::read_to_string("../ejemplos/ej1.txt")
        .expect("No se pudo leer el archivo");

    let grammar = Grammar::from_string(&input);

    println!("Símbolo inicial: {}", grammar.start_symbol);

    println!("\nNo terminales:");
    for nt in &grammar.non_terminals {
        println!("  {}", nt);
    }

    println!("\nTerminales:");
    for t in &grammar.terminals {
        println!("  {}", t);
    }

    println!("\nProducciones:");
    for p in &grammar.productions {
        println!("  {} -> {}", p.lhs, p.rhs.join(" "));
    }
}