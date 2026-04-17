use std::collections::HashSet;
use std::error::Error;
use std::fmt;

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
    #[allow(dead_code)]
    pub terminals: HashSet<Symbol>,
    pub productions: Vec<Production>,
}

impl Grammar {
    #[allow(dead_code)]
    pub fn new(start_symbol: &str) -> Self {
        let mut non_terminals = HashSet::new();
        non_terminals.insert(start_symbol.trim().to_string());

        Self {
            start_symbol: start_symbol.trim().to_string(),
            non_terminals,
            terminals: HashSet::new(),
            productions: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_production(&mut self, lhs: &str, rhs: Vec<Symbol>) {
        let lhs = lhs.trim();
        if lhs.is_empty() {
            return;
        }

        self.productions.push(Production {
            lhs: lhs.to_string(),
            rhs,
        });

        // Keep symbol sets consistent even if productions are added incrementally.
        self.recompute_symbols();
    }

    #[allow(dead_code)]
    fn recompute_symbols(&mut self) {
        let mut non_terminals = HashSet::new();
        for production in &self.productions {
            non_terminals.insert(production.lhs.clone());
        }

        let mut terminals = HashSet::new();
        for production in &self.productions {
            for symbol in &production.rhs {
                if !non_terminals.contains(symbol) {
                    terminals.insert(symbol.clone());
                }
            }
        }

        self.non_terminals = non_terminals;
        self.terminals = terminals;
    }

    #[allow(dead_code)]
    pub fn from_string(input: &str) -> Self {
        Self::try_from_string(input).expect("Gramática inválida")
    }

    pub fn get_productions_for(&self, lhs: &str) -> Vec<&Production> {
        self.productions
            .iter()
            .filter(|p| p.lhs == lhs)
            .collect()
    }

    pub fn is_nonterminal(&self, symbol: &str) -> bool {
        self.non_terminals.contains(symbol)
    }

    pub fn augmented(&self) -> Self {
        let mut new_grammar = self.clone();
        let augmented_start = format!("{}'", self.start_symbol);

        new_grammar.non_terminals.insert(augmented_start.clone());

        new_grammar.productions.insert(
            0,
            Production {
                lhs: augmented_start.clone(),
                rhs: vec![self.start_symbol.clone()],
            },
        );

        new_grammar.start_symbol = augmented_start;
        new_grammar
    }

    pub fn try_from_string(input: &str) -> Result<Self, GrammarError> {
        let mut productions = Vec::new();
        let mut non_terminals = HashSet::new();

        let mut lines: Vec<(usize, String)> = Vec::new();
        for (idx, raw) in input.lines().enumerate() {
            let mut line = raw.trim().to_string();

            // Skip empty lines and full-line comments.
            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') || line.starts_with("//") || line.starts_with(';') {
                continue;
            }

            // Allow trailing comments with " #" (common in examples).
            if let Some((before, _after)) = line.split_once(" #") {
                line = before.trim().to_string();
                if line.is_empty() {
                    continue;
                }
            }

            lines.push((idx + 1, line));
        }

        if lines.is_empty() {
            return Err(GrammarError::EmptyGrammar);
        }

        // Detect non-terminals from every LHS.
        for (line_no, line) in &lines {
            let (lhs_raw, _rhs_raw) = split_production(line_no, line)?;
            let lhs = lhs_raw.trim();
            if lhs.is_empty() {
                return Err(GrammarError::InvalidProduction {
                    line_no: *line_no,
                    line: line.clone(),
                    message: "LHS vacío".to_string(),
                });
            }

            non_terminals.insert(lhs.to_string());
        }

        // Build productions.
        for (line_no, line) in &lines {
            let (lhs_raw, rhs_raw) = split_production(line_no, line)?;
            let lhs = lhs_raw.trim().to_string();
            let rhs_all = rhs_raw.trim();

            for alternative_raw in rhs_all.split('|') {
                let alternative = alternative_raw.trim();

                // Empty alternative => epsilon.
                if alternative.is_empty() {
                    productions.push(Production {
                        lhs: lhs.clone(),
                        rhs: Vec::new(),
                    });
                    continue;
                }

                let mut rhs: Vec<String> = alternative
                    .split_whitespace()
                    .map(|s| s.trim().to_string())
                    .collect();

                // Normalize epsilon representations.
                if rhs.len() == 1 && (rhs[0] == "ε" || rhs[0].eq_ignore_ascii_case("epsilon")) {
                    rhs.clear();
                } else if rhs.iter().any(|s| s == "ε" || s.eq_ignore_ascii_case("epsilon")) {
                    return Err(GrammarError::InvalidProduction {
                        line_no: *line_no,
                        line: line.clone(),
                        message: "ε/epsilon debe ser la única cosa en una alternativa".to_string(),
                    });
                }

                productions.push(Production { lhs: lhs.clone(), rhs });
            }
        }

        if productions.is_empty() {
            return Err(GrammarError::EmptyGrammar);
        }

        // Choose start symbol.
        let start_symbol = productions[0].lhs.clone();

        // Detect terminals.
        let mut terminals = HashSet::new();
        for production in &productions {
            for symbol in &production.rhs {
                if !non_terminals.contains(symbol) {
                    terminals.insert(symbol.clone());
                }
            }
        }

        Ok(Grammar {
            start_symbol,
            non_terminals,
            terminals,
            productions,
        })
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rhs.is_empty() {
            write!(f, "{} -> ε", self.lhs)
        } else {
            write!(f, "{} -> {}", self.lhs, self.rhs.join(" "))
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, p) in self.productions.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}

fn split_production<'a>(line_no: &usize, line: &'a str) -> Result<(&'a str, &'a str), GrammarError> {
    match line.split_once("->") {
        Some((lhs, rhs)) => Ok((lhs, rhs)),
        None => Err(GrammarError::InvalidProduction {
            line_no: *line_no,
            line: line.to_string(),
            message: "Falta '->'".to_string(),
        }),
    }
}

#[derive(Debug, Clone)]
pub enum GrammarError {
    EmptyGrammar,
    InvalidProduction { line_no: usize, line: String, message: String },
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrammarError::EmptyGrammar => write!(f, "La gramática está vacía (no hay producciones)."),
            GrammarError::InvalidProduction {
                line_no,
                line,
                message,
            } => {
                write!(
                    f,
                    "Producción inválida (línea {}): {}\nMotivo: {}",
                    line_no, line, message
                )
            }
        }
    }
}

impl Error for GrammarError {}