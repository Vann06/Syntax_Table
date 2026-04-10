use std::collections::HashSet;

pub type Symbol = String;

#[derive(Debug, Clone)]
pub struct Production {
    pub lhs: Symbol, // left-hand side
    pub rhs: Vec<Symbol>, // right-hand side
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub start_symbol: Symbol,
    pub non_terminals: HashSet<Symbol>,
    pub terminals: HashSet<Symbol>,
    pub productions: Vec<Production>,
}


// Leer el archivo 

impl Grammar {
    pub fn from_string(input: &str) -> Self {
        let mut productions = Vec::new();
        let mut non_terminals = HashSet::new();

        let lines: Vec<&str> = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();
        // detección de no terminales
        for line in &lines {
            let parts: Vec<&str> = line.split("->").collect();

            if parts.len() != 2 {
                panic!("Producción inválida: {}", line);
            }

            let lhs = parts[0].trim().to_string();
            non_terminals.insert(lhs);
        }

        // construcción de producciones
        for line in &lines {
            let parts: Vec<&str> = line.split("->").collect();
            let lhs = parts[0].trim().to_string();
            let rhs_all = parts[1].trim();

            for alternative in rhs_all.split('|') {
                let rhs: Vec<String> = alternative
                    .split_whitespace()
                    .map(|s| s.trim().to_string())
                    .collect();

                productions.push(Production {
                    lhs: lhs.clone(),
                    rhs,
                });
            }
        }

        // Elegir el simbolo inicial 
        let start_symbol = productions
            .first()
            .expect("La gramática no puede estar vacía")
            .lhs
            .clone();

        // Detección de terminales
        let mut terminals = HashSet::new();

        for production in &productions {
            for symbol in &production.rhs {
                if symbol != "ε" && !non_terminals.contains(symbol) {
                    terminals.insert(symbol.clone());
                }
            }
        }

        Grammar {
            start_symbol,
            non_terminals,
            terminals,
            productions,
        }
    }
}