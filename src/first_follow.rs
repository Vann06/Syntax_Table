use std::collections::{HashMap, HashSet};

use crate::grammar::{Grammar, Symbol};

// Casos a saber de First 
pub fn first_of_sequence(
    seq: &[Symbol],
    grammar: &Grammar,
    first_sets: &HashMap<Symbol, HashSet<Symbol>>,
) -> HashSet<Symbol> {
    let mut result = HashSet::new();

    // Caso 1: secuencia vacía
    if seq.is_empty() {
        result.insert("ε".to_string());
        return result;
    }

    let mut all_can_be_epsilon = true;

    for symbol in seq {
        // Caso 2: epsilon explícito
        if symbol == "ε" {
            result.insert("ε".to_string());
            return result;
        }

        // Caso 3: terminal
        if grammar.terminals.contains(symbol) {
            result.insert(symbol.clone());
            all_can_be_epsilon = false;
            break;
        }

        // Caso 4: no terminal
        if grammar.non_terminals.contains(symbol) {
            if let Some(first_symbol) = first_sets.get(symbol) {
                for item in first_symbol {
                    if item != "ε" {
                        result.insert(item.clone());
                    }
                }

                if !first_symbol.contains("ε") {
                    all_can_be_epsilon = false;
                    break;
                }
            } else {
                all_can_be_epsilon = false;
                break;
            }
        } else {
            // Caso 5: símbolo desconocido
            result.insert(symbol.clone());
            all_can_be_epsilon = false;
            break;
        }
    }

    if all_can_be_epsilon {
        result.insert("ε".to_string());
    }

    result
}