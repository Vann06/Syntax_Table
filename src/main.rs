mod grammar;
mod item;
mod parser;
mod closure;
mod ui;

use closure::closure;
use item::Item;
use parser::{load_grammar_from_file, parse_item};
use ui::{list_grammar_files, read_line, read_usize};

fn main() {
    let folder = "grammars";

    let files = match list_grammar_files(folder) {
        Ok(files) if !files.is_empty() => files,
        Ok(_) => {
            println!("No se encontraron archivos de gramática.");
            return;
        }
        Err(e) => {
            println!("Error leyendo la carpeta de gramáticas: {}", e);
            return;
        }
    };

    println!("Gramáticas disponibles:");
    for (i, file) in files.iter().enumerate() {
        println!("{}. {}", i + 1, file);
    }

    let choice = read_usize("Seleccione una gramática: ");
    if choice == 0 || choice > files.len() {
        println!("Selección inválida.");
        return;
    }

    let selected_file = &files[choice - 1];
    let path = format!("{}/{}", folder, selected_file);

    let grammar = match load_grammar_from_file(&path) {
        Ok(g) => g,
        Err(e) => {
            println!("Error cargando la gramática: {}", e);
            return;
        }
    };

    println!("\nGramática original:");
    println!("{}", grammar);

    let augmented = grammar.augmented();

    println!("Gramática aumentada:");
    println!("{}", augmented);

    println!("¿Qué desea hacer?");
    println!("1. Calcular cerradura del ítem inicial automático");
    println!("2. Ingresar un ítem manualmente");

    let option = read_usize("Seleccione una opción: ");

    let initial_item = match option {
        1 => {
            let start_production = augmented
                .productions
                .first()
                .expect("La gramática aumentada no tiene producciones");
            Item::new(&start_production.lhs, start_production.rhs.clone(), 0)
        }
        2 => {
            let input = read_line("Ingrese el ítem (ejemplo: S -> . S S +): ");
            match parse_item(&input) {
                Ok(item) => item,
                Err(e) => {
                    println!("Error parseando el ítem: {}", e);
                    return;
                }
            }
        }
        _ => {
            println!("Opción inválida.");
            return;
        }
    };

    println!("\nÍtem de entrada:");
    println!("{}", initial_item);

    let result = closure(&augmented, vec![initial_item.clone()]);

    println!("\nÍtems agregados durante la cerradura:");
    if result.added_items.is_empty() {
        println!("No se agregaron nuevos ítems.");
    } else {
        for item in &result.added_items {
            println!("{}", item);
        }
    }

    println!("\nCerradura final:");
    let mut final_items: Vec<_> = result.final_items.iter().collect();
    final_items.sort_by(|a, b| {
        a.lhs
            .cmp(&b.lhs)
            .then(a.dot.cmp(&b.dot))
            .then(a.rhs.join(" ").cmp(&b.rhs.join(" ")))
    });

    for item in final_items {
        println!("{}", item);
    }
}