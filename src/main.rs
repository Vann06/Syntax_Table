mod grammar;
mod first_follow;

use std::fs;

use grammar::Grammar;
use first_follow::{compute_first_sets, compute_follow_sets};

fn main() {
    let input = fs::read_to_string("./ejemplos/ej1.txt")
        .expect("No se pudo leer el archivo");

    let grammar = Grammar::from_string(&input);

    // Mostrar la gramática
    println!("═══════════════════════════════════════");
    println!("          GRAMÁTICA");
    println!("═══════════════════════════════════════");
    println!("Símbolo inicial: {}", grammar.start_symbol);
    println!("\nNo terminales: {:?}", grammar.non_terminals);
    println!("Terminales: {:?}", grammar.terminals);
    println!("\nProducciones:");
    for production in &grammar.productions {
        println!("  {} -> {}", production.lhs, production.rhs.join(" "));
    }

    // Calcular FIRST
    println!("\n═══════════════════════════════════════");
    println!("        CONJUNTOS FIRST");
    println!("═══════════════════════════════════════");
    let first_sets = compute_first_sets(&grammar);

    // Mostrar FIRST ordenado por no-terminal
    let mut sorted_nonterminals: Vec<_> = grammar.non_terminals.iter().collect();
    sorted_nonterminals.sort();

    for nt in sorted_nonterminals {
        if let Some(first_set) = first_sets.get(nt) {
            let mut sorted_set: Vec<String> = first_set.iter().map(|s| s.clone()).collect();
            sorted_set.sort();
            println!("FIRST({}) = {{ {} }}", nt, sorted_set.join(", "));
        }
    }

    // Calcular FOLLOW
    println!("\n═══════════════════════════════════════");
    println!("       CONJUNTOS FOLLOW");
    println!("═══════════════════════════════════════");
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Mostrar FOLLOW ordenado por no-terminal
    let mut sorted_nonterminals: Vec<_> = grammar.non_terminals.iter().collect();
    sorted_nonterminals.sort();

    for nt in sorted_nonterminals {
        if let Some(follow_set) = follow_sets.get(nt) {
            let mut sorted_set: Vec<String> = follow_set.iter().map(|s| s.clone()).collect();
            sorted_set.sort();
            println!("FOLLOW({}) = {{ {} }}", nt, sorted_set.join(", "));
        }
    }

    println!("\n═══════════════════════════════════════");
}