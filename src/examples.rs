mod grammar;
mod first_follow;
mod parsing_table;
mod examples;

use std::collections::HashSet;

use examples::{arithmetic_grammar, grammar_three, grammar_two};
use first_follow::{compute_first, compute_follow};
use grammar::Grammar;
use parsing_table::{build_parsing_table, conflicts, is_ll1};

fn print_set(name: &str, items: &HashSet<String>) {
    let mut v: Vec<_> = items.iter().cloned().collect();
    v.sort();
    println!("{} = {{ {} }}", name, v.join(", "));
}

fn print_grammar(g: &Grammar) {
    println!("Gramática:");
    for p in &g.productions {
        println!("  {} -> {}", p.lhs, p.rhs.join(" "));
    }
}

fn run_grammar(title: &str, input: &str) {
    println!("\n==============================");
    println!("{}", title);
    println!("==============================");

    let grammar = Grammar::from_string(input);
    print_grammar(&grammar);

    let first = compute_first(&grammar);
    let follow = compute_follow(&grammar, &first);

    println!("\nFIRST:");
    let mut nts: Vec<_> = grammar.non_terminals.iter().cloned().collect();
    nts.sort();
    for nt in &nts {
        print_set(&format!("FIRST({})", nt), first.get(nt).unwrap());
    }

    println!("\nFOLLOW:");
    for nt in &nts {
        print_set(&format!("FOLLOW({})", nt), follow.get(nt).unwrap());
    }

    let table = build_parsing_table(&grammar, &first, &follow);

    println!("\n¿Es LL(1)? {}", if is_ll1(&table) { "Sí" } else { "No" });

    let confs = conflicts(&table);
    if !confs.is_empty() {
        println!("Conflictos detectados:");
        for ((nt, term), prods) in confs {
            println!("  Conflicto en [{}, {}]:", nt, term);
            for p in prods {
                println!("    {} -> {}", p.lhs, p.rhs.join(" "));
            }
        }
    }

    println!("\nTabla predictiva:");
    let mut terminals: Vec<_> = grammar.terminals.iter().cloned().collect();
    terminals.sort();
    terminals.push("$".to_string());

    print!("{:10}", "");
    for t in &terminals {
        print!("{:20}", t);
    }
    println!();

    for nt in &nts {
        print!("{:10}", nt);
        for t in &terminals {
            if let Some(prods) = table.get(&(nt.clone(), t.clone())) {
                let text = prods
                    .iter()
                    .map(|p| format!("{} -> {}", p.lhs, p.rhs.join(" ")))
                    .collect::<Vec<_>>()
                    .join(" | ");
                print!("{:20}", text);
            } else {
                print!("{:20}", "");
            }
        }
        println!();
    }
}

fn main() {
    run_grammar("Gramática 1: Expresiones Aritméticas", arithmetic_grammar());
    run_grammar("Gramática 2", grammar_two());
    run_grammar("Gramática 3", grammar_three());
}