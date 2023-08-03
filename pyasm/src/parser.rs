use std::collections::{HashMap, VecDeque};

use crate::Commands;


pub fn tok_to_commands(tokens: Vec<String>) -> Vec<(Commands, Vec<String>)> {
    let mut commands: Vec<(Commands, Vec<String>)> = Vec::new();
    let mut unique_nb: u64 = 0;
    let mut mess_nb: u64 = 0;
    let mut states: VecDeque<(Commands, u64)> = VecDeque::new();

    for token in tokens {
        if token == "end" {
            match states.pop_back() {
                Some((Commands::If, nb)) => {
                    commands.push((Commands::EndIf, [nb.to_string()].to_vec()));
                },
                Some((Commands::While, nb)) => {
                    commands.push((Commands::EndWhile, [nb.to_string()].to_vec()));
                },
                _ => {
                    println!("Error : end");
                }
            }
        }
        else if token.chars().all(char::is_numeric) {
            commands.push((Commands::Push, vec![token]));
        }
        else if token == "." {
            commands.push((Commands::Dump, vec![]));
        }
        else if token == "+" {
            commands.push((Commands::Add, vec![]));
        }
        else if token == "-" {
            commands.push((Commands::Sub, vec![]));
        }
        else if token == "dup" {
            commands.push((Commands::Dup, vec![]));
        }
        else if token == "if" {
            commands.push((Commands::If, vec![unique_nb.to_string()]));
            states.push_back((Commands::If, unique_nb));
            unique_nb += 1;
        }
        else if token == "while" {
            commands.push((Commands::While, vec![unique_nb.to_string()]));
            states.push_back((Commands::While, unique_nb));
            unique_nb += 1;
        }
        else if token == ">" {
            commands.push((Commands::G, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "<" {
            commands.push((Commands::L, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "=" {
            commands.push((Commands::E, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "!=" {
            commands.push((Commands::Ne, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == ">=" {
            commands.push((Commands::Ge, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "<=" {
            commands.push((Commands::Le, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if is_string(&token) {
            commands.push((Commands::PrintStringConst, [token, format!("{}", mess_nb)].to_vec()));
            mess_nb += 1;
        }
        // si le string commence par syscall
        else if token.starts_with("syscall") {
            // on récupère le nombre après syscall
            let nb = token.chars().skip(7).collect::<String>();
            // on vérifie que c'est bien un nombre
            if nb.chars().all(char::is_numeric) {
                // on convertit le nombre en u64
                let nb = nb.parse::<u64>().unwrap();
                // on ajoute la commande syscall
                commands.push((Commands::Syscall, [nb.to_string()].to_vec()));
            }
            else {
                println!("Error : syscall invoqued without a number");
            }
        }
        else if token == "*" {
            commands.push((Commands::Mul, vec![]));
        }
        else if token == "mem" {
            commands.push((Commands::Mem, vec![]));
        }
        else if token == "read8" {
            commands.push((Commands::Read8, vec![]));
        }
        else if token == "store8" {
            commands.push((Commands::Store8, vec![]));
        }
        else {
            println!("Error : token: {}", token);
        }
    }
    commands
}

pub fn parse_macros(tokens: Vec<String>) -> Vec<String> {
    let mut new_tokens: Vec<String> = Vec::new();
    let mut macros: HashMap<String, Vec<String>> = HashMap::new();
    let mut current: Vec<String> = Vec::new();
    let mut end_counter = 0;
    let mut in_macro: bool = false;

    for token in tokens.clone() {
        if token == "macro" {
            in_macro = true;
        }
        else if token == "if" || token == "while" {
            end_counter += 1;
        }
        else if token == "end" {
            if end_counter == 0 {
                in_macro = false;
                macros.insert(current[1].clone(), current[2..].to_vec());
                current.clear();
            }
            else {
                end_counter -= 1;
            }
        }
        if in_macro {
            current.push(token.clone());
        }
    }

    for token in tokens {
        if macros.contains_key(&token) {
            new_tokens.append(&mut macros[&token].clone());
        }
        else {
            new_tokens.push(token);
        }
    }

    let mut true_tokens: Vec<String> = Vec::new();

    // we remove the macro definitions
    let mut in_macro: bool = false;
    let mut end_counter: i32 = 0;
    for token in new_tokens.clone() {
        if token == "macro" {
            in_macro = true;
        }
        if !in_macro {
            true_tokens.push(token.clone());
        }
        if token == "if" || token == "while" {
            end_counter += 1;
        }
        else if token == "end" {
            if end_counter == 0 {
                in_macro = false;
            }
            else {
                end_counter -= 1;
            }
        }
    }

    true_tokens
}

fn is_string(token: &str) -> bool {
    if let Some(first_char) = token.chars().next() {
        if let Some(last_char) = token.chars().rev().next() {
            return first_char == '"' && last_char == '"';
        }
    }
    false
}

pub fn cut_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
