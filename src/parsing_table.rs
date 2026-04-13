use std::collections::{HashMap, HashSet};

use crate::first_follow::first_of_sequence;
use crate::grammar::{Grammar, Production, Symbol};

pub type ParsingTable = HashMap<(Symbol, Symbol), Vec<Production>>;

pub fn build_parsing_table(
    grammar: &Grammar,
    first_sets: &HashMap<Symbol, HashSet<Symbol>>,
    follow_sets: &HashMap<Symbol, HashSet<Symbol>>,
) -> ParsingTable {
    let mut table: ParsingTable = HashMap::new();

    for production in &grammar.productions {
        let lhs = production.lhs.clone();
        let rhs = &production.rhs;

        let first_alpha = first_of_sequence(rhs, grammar, first_sets);

        for terminal in &first_alpha {
            if terminal == "ε" {
                continue;
            }

            table
                .entry((lhs.clone(), terminal.clone()))
                .or_default()
                .push(production.clone());
        }

        if first_alpha.contains("ε") {
            if let Some(follow_lhs) = follow_sets.get(&lhs) {
                for lookahead in follow_lhs {
                    table
                        .entry((lhs.clone(), lookahead.clone()))
                        .or_default()
                        .push(production.clone());
                }
            }
        }
    }

    table
}

pub fn is_ll1(table: &ParsingTable) -> bool {
    table.values().all(|prods| prods.len() <= 1)
}

pub fn find_conflicts(table: &ParsingTable) -> Vec<((Symbol, Symbol), Vec<Production>)> {
    let mut conflicts: Vec<_> = table
        .iter()
        .filter_map(|((nt, term), prods)| {
            if prods.len() > 1 {
                Some(((nt.clone(), term.clone()), prods.clone()))
            } else {
                None
            }
        })
        .collect();

    conflicts.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));
    conflicts
}

fn rhs_text(p: &Production) -> String {
    if p.rhs.len() == 1 && p.rhs[0] == "ε" {
        "ε".to_string()
    } else {
        p.rhs.join(" ")
    }
}

fn cell_text(prods: &[Production]) -> String {
    match prods.len() {
        0 => String::new(),
        1 => rhs_text(&prods[0]),
        _ => prods
            .iter()
            .map(rhs_text)
            .collect::<Vec<_>>()
            .join(" | "),
    }
}

fn clamp_usize(value: usize, min: usize, max: usize) -> usize {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn truncate_to_width(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let len = text.chars().count();
    if len <= width {
        return text.to_string();
    }

    if width <= 3 {
        return text.chars().take(width).collect();
    }

    let mut out: String = text.chars().take(width - 3).collect();
    out.push_str("...");
    out
}

pub fn print_parsing_table(grammar: &Grammar, table: &ParsingTable) {
    let mut non_terminals: Vec<_> = grammar.non_terminals.iter().cloned().collect();
    non_terminals.sort();

    let mut terminals: Vec<_> = grammar.terminals.iter().cloned().collect();
    terminals.sort();
    terminals.push("$".to_string());

    let max_nt_len = non_terminals.iter().map(|s| s.chars().count()).max().unwrap_or(1);
    let max_term_len = terminals.iter().map(|s| s.chars().count()).max().unwrap_or(1);

    // ~80-column friendly defaults.
    let row_w = clamp_usize(max_nt_len, 3, 8);
    let col_w = clamp_usize(std::cmp::max(7, max_term_len), 7, 10);

    let total_w = 2 + row_w + (terminals.len() * (3 + col_w));
    let divider = "-".repeat(total_w);

    println!("{}", divider);

    // Header
    print!("| {:<row_w$}", "", row_w = row_w);
    for t in &terminals {
        let t = truncate_to_width(t, col_w);
        print!(" | {:<col_w$}", t, col_w = col_w);
    }
    println!(" |");

    println!("{}", divider);

    for nt in &non_terminals {
        print!("| {:<row_w$}", nt, row_w = row_w);
        for t in &terminals {
            let text = table
                .get(&(nt.clone(), t.clone()))
                .map(|prods| cell_text(prods))
                .unwrap_or_default();
            let text = truncate_to_width(&text, col_w);
            print!(" | {:<col_w$}", text, col_w = col_w);
        }
        println!(" |");
    }

    println!("{}", divider);
}
