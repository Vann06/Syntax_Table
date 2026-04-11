use std::collections::{HashMap, HashSet};

use crate::grammar::{Grammar, Symbol};

// Casos a saber de First

pub fn compute_first_sets(grammar: &Grammar) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut first_sets: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();

    // Inicializar FIRST para todos los no terminales
    for non_terminal in &grammar.non_terminals {
        first_sets.insert(non_terminal.clone(), HashSet::new());
    }

    // Agregar terminales a FIRST(terminal) = {terminal}
    for terminal in &grammar.terminals {
        let mut set = HashSet::new();
        set.insert(terminal.clone());
        first_sets.insert(terminal.clone(), set);
    }

    // Algoritmo iterativo para calcular FIRST
    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            let lhs = &production.lhs;
            let rhs = &production.rhs;

            // Obtener el conjunto FIRST actual del lado izquierdo
            let current_first = first_sets.get_mut(lhs).unwrap().clone();
            let mut new_symbols = HashSet::new();

            // Procesar el lado derecho
            let mut all_can_be_epsilon = true;

            for symbol in rhs {
                // Si el símbolo es epsilon, agregar epsilon
                if symbol == "ε" {
                    new_symbols.insert("ε".to_string());
                    all_can_be_epsilon = false;
                    break;
                }

                let symbol_first = first_sets.get(symbol).cloned().unwrap_or_default();

                // Agregar todos los símbolos de FIRST(símbolo) excepto ε
                for item in &symbol_first {
                    if item != "ε" && !current_first.contains(item) {
                        new_symbols.insert(item.clone());
                        changed = true;
                    }
                }

                // Si ε no está en FIRST(símbolo), parar
                if !symbol_first.contains("ε") {
                    all_can_be_epsilon = false;
                    break;
                }
            }

            // Si todos los símbolos pueden ser ε, agregar ε a FIRST(lhs)
            if all_can_be_epsilon && !current_first.contains("ε") {
                new_symbols.insert("ε".to_string());
                changed = true;
            }

            // Actualizar el conjunto FIRST
            for symbol in new_symbols {
                first_sets.get_mut(lhs).unwrap().insert(symbol);
            }
        }
    }

    first_sets
}

pub fn compute_follow_sets(
    grammar: &Grammar,
    first_sets: &HashMap<Symbol, HashSet<Symbol>>,
) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut follow_sets: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();

    // Inicializar FOLLOW para todos los no terminales
    for non_terminal in &grammar.non_terminals {
        follow_sets.insert(non_terminal.clone(), HashSet::new());
    }

    // Agregar $ al FOLLOW del símbolo inicial
    follow_sets
        .get_mut(&grammar.start_symbol)
        .unwrap()
        .insert("$".to_string());

    // Algoritmo iterativo para calcular FOLLOW
    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            let lhs = &production.lhs;
            let rhs = &production.rhs;

            // Para cada símbolo en el RHS
            for (i, symbol) in rhs.iter().enumerate() {
                // Solo procesamos si es un no-terminal
                if !grammar.non_terminals.contains(symbol) {
                    continue;
                }

                let rest_of_sequence = &rhs[i + 1..];
                let first_rest = first_of_sequence(rest_of_sequence, grammar, first_sets);

                // Agregar todos los símbolos de FIRST(resto) excepto ε
                for item in &first_rest {
                    if item != "ε" {
                        let was_inserted = follow_sets.get_mut(symbol).unwrap().insert(item.clone());
                        if was_inserted {
                            changed = true;
                        }
                    }
                }

                // Si ε está en FIRST(resto), agregar FOLLOW(lhs) a FOLLOW(símbolo)
                if first_rest.contains("ε") {
                    if let Some(lhs_follow) = follow_sets.get(lhs).cloned() {
                        for item in lhs_follow {
                            let was_inserted = follow_sets.get_mut(symbol).unwrap().insert(item);
                            if was_inserted {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    follow_sets
}


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