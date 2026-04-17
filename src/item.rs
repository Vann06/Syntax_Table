// definición del item LR(0)

/*
Se necesita guardar:
- lado izquierdo 
- lado derecho
- puntito
*/
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub lhs: String,
    pub rhs: Vec<String>,
    pub dot: usize,
}

impl Item {
    pub fn new(lhs: &str, rhs: Vec<String>, dot: usize) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs,
            dot,
        }
    }

    pub fn symbol_after_dot(&self) -> Option<&String> {
        if self.dot < self.rhs.len() {
            Some(&self.rhs[self.dot])
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.dot >= self.rhs.len()
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        for (i, symbol) in self.rhs.iter().enumerate() {
            if i == self.dot {
                parts.push(".".to_string());
            }
            parts.push(symbol.clone());
        }

        if self.dot == self.rhs.len() {
            parts.push(".".to_string());
        }

        write!(f, "{} -> {}", self.lhs, parts.join(" "))
    }
}