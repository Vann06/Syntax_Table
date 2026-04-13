mod grammar;
mod first_follow;
mod parsing_table;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use grammar::Grammar;
use first_follow::{compute_first_sets, compute_follow_sets};
use parsing_table::{build_parsing_table, find_conflicts, is_ll1, print_parsing_table};

fn rhs_text(rhs: &[String]) -> String {
    if rhs.is_empty() {
        "ε".to_string()
    } else {
        rhs.join(" ")
    }
}

fn run_grammar_from_file(path: &Path) {
    println!("\n═══════════════════════════════════════");
    println!("ARCHIVO: {}", path.display());
    println!("═══════════════════════════════════════");

    let input = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No se pudo leer el archivo '{}': {}", path.display(), e);
            return;
        }
    };

    let grammar = match Grammar::try_from_string(&input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error al parsear la gramática en '{}':\n{}", path.display(), e);
            return;
        }
    };

    // Mostrar la gramática
    println!("═══════════════════════════════════════");
    println!("          GRAMÁTICA");
    println!("═══════════════════════════════════════");
    println!("Símbolo inicial: {}", grammar.start_symbol);

    let mut non_terminals: Vec<_> = grammar.non_terminals.iter().cloned().collect();
    non_terminals.sort();
    let mut terminals: Vec<_> = grammar.terminals.iter().cloned().collect();
    terminals.sort();

    println!("\nNo terminales: {:?}", non_terminals);
    println!("Terminales: {:?}", terminals);
    println!("\nProducciones:");
    for production in &grammar.productions {
        println!("  {} -> {}", production.lhs, rhs_text(&production.rhs));
    }

    // Calcular FIRST
    println!("\n═══════════════════════════════════════");
    println!("        CONJUNTOS FIRST");
    println!("═══════════════════════════════════════");
    let first_sets = compute_first_sets(&grammar);

    // Mostrar FIRST ordenado por no-terminal
    let mut sorted_nonterminals: Vec<_> = grammar.non_terminals.iter().collect();
    sorted_nonterminals.sort();

    for nt in &sorted_nonterminals {
        if let Some(first_set) = first_sets.get(*nt) {
            let mut sorted_set: Vec<String> = first_set.iter().cloned().collect();
            sorted_set.sort();
            println!("FIRST({}) = {{ {} }}", nt, sorted_set.join(", "));
        }
    }

    // Calcular FOLLOW
    println!("\n═══════════════════════════════════════");
    println!("       CONJUNTOS FOLLOW");
    println!("═══════════════════════════════════════");
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    for nt in &sorted_nonterminals {
        if let Some(follow_set) = follow_sets.get(*nt) {
            let mut sorted_set: Vec<String> = follow_set.iter().cloned().collect();
            sorted_set.sort();
            println!("FOLLOW({}) = {{ {} }}", nt, sorted_set.join(", "));
        }
    }

    // Tabla predictiva LL(1)
    println!("\n═══════════════════════════════════════");
    println!("      TABLA PREDICTIVA LL(1)");
    println!("═══════════════════════════════════════");

    let table = build_parsing_table(&grammar, &first_sets, &follow_sets);
    println!("¿Es LL(1)? {}", if is_ll1(&table) { "Sí" } else { "No" });

    let conflicts = find_conflicts(&table);
    if conflicts.is_empty() {
        println!("Conflictos: ninguno");
    } else {
        println!("Conflictos detectados (celdas con más de una producción):");
        for ((nt, term), prods) in conflicts {
            println!("  Conflicto en [{}, {}]:", nt, term);
            for p in prods {
                println!("    {} -> {}", p.lhs, rhs_text(&p.rhs));
            }
        }
    }

    println!("\nTabla predictiva:");
    print_parsing_table(&grammar, &table);
}

fn collect_txt_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext.eq_ignore_ascii_case("txt") {
                    out.push(path);
                }
            }
        }
    }

    out.sort();
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Behavior:
    // - No args: run all ./ejemplos/*.txt
    // - With args: treat each arg as a path to a .txt file and run them in order.
    if args.is_empty() {
        let dir = Path::new("./ejemplos");
        let files = collect_txt_files_in_dir(dir);
        if files.is_empty() {
            eprintln!("No se encontraron archivos .txt en {}", dir.display());
            return;
        }

        for file in files {
            run_grammar_from_file(&file);
        }
    } else {
        for arg in args {
            let path = Path::new(&arg);
            if path.is_dir() {
                let files = collect_txt_files_in_dir(path);
                if files.is_empty() {
                    eprintln!("No se encontraron archivos .txt en {}", path.display());
                    continue;
                }

                for file in files {
                    run_grammar_from_file(&file);
                }
            } else {
                run_grammar_from_file(path);
            }
        }
    }
}