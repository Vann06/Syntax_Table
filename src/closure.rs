
use std::collections::{HashSet, VecDeque};

use crate::grammar::Grammar;
use crate::item::Item;
/*
CLOSURE
- recibir gramatica
- recibe item
- usa una cola-
- va agregando items a la cola
- devuelve cerradura completa
- historail de los items agregados 

*/


pub struct ClosureResult {
    pub final_items: HashSet<Item>,
    pub added_items: Vec<Item>,
}

pub fn closure(grammar: &Grammar, initial_items: Vec<Item>) -> ClosureResult {
    let mut result: HashSet<Item> = HashSet::new();
    let mut queue: VecDeque<Item> = VecDeque::new();
    let mut added_items: Vec<Item> = Vec::new();

    for item in initial_items {
        if result.insert(item.clone()) {
            queue.push_back(item);
        }
    }

    while let Some(item) = queue.pop_front() {
        if let Some(symbol) = item.symbol_after_dot() {
            if grammar.is_nonterminal(symbol) {
                for production in grammar.get_productions_for(symbol) {
                    let new_item = Item {
                        lhs: production.lhs.clone(),
                        rhs: production.rhs.clone(),
                        dot: 0,
                    };

                    if result.insert(new_item.clone()) {
                        queue.push_back(new_item.clone());
                        added_items.push(new_item);
                    }
                }
            }
        }
    }

    ClosureResult {
        final_items: result,
        added_items,
    }
}