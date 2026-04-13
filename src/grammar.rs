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
    pub terminals: HashSet<Symbol>,
    pub productions: Vec<Production>,
}

impl Grammar {
    #[allow(dead_code)]
    pub fn from_string(input: &str) -> Self {
        Self::try_from_string(input).expect("Gramática inválida")
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