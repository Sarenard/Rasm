use std::collections::VecDeque;
use std::process::Command;
use std::io::prelude::*;
use std::fs::File;
use std::env;

macro_rules! command_enum {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        #[derive(PartialEq)]
        enum Commands {
            $($variant),*
        }

        impl std::fmt::Display for Commands {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Commands::$variant => write!(f, stringify!($variant))),*
                }
            }
        }
    };
}

command_enum!(
    Push, 
    Dump, 
    Add, 
    Dup, 
    If, 
    EndIf, 
    Sub, 
    While, 
    EndWhile, 
    G,
    L,
    E,
    Ne,
    Ge,
    Le,
    PrintStringConst // temporary
);

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: pyasm -f <input file>");
        return Ok(());
    }

    let input_file = &args[2];

    make_asm(input_file).expect("failed to make asm");

    let _nasm_output = Command::new("nasm")
        .args(["-f", "elf64", "output/output.asm", "-o", "output/output.o"])
        .output()
        .expect("failed to execute process");
    if !_nasm_output.status.success() {
        println!("nasm output: {:?}", _nasm_output);
        return Ok(());
    }
    let _ld_output = Command::new("ld")
        .args(["-m", "elf_x86_64", "output/output.o", "-o", "output/output"])
        .output()
        .expect("failed to execute process");
    if !_ld_output.status.success() {
        println!("ld output: {:?}", _ld_output);
        return Ok(());
    }
    Ok(())
}

fn make_asm(input_file: &str) -> std::io::Result<()> {
    let mut file = File::create("output/output.asm")?;
    let mut program = String::new();
    
    let mut input_file = File::open(input_file)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let tokens = file_to_tok(&contents.to_string());
    #[cfg(debug_assertions)]
    println!("tokens: {:?}", tokens);
    
    let commands = tok_to_commands(tokens);
    #[cfg(debug_assertions)]
    println!("commands: {:?}", commands);

    // si jamais il y a au moins un PrintStringConst, on ajoute le .data
    if commands.iter().any(|x| x.0 == Commands::PrintStringConst) {
        program.push_str("section .data\n");
    }
    // on ajoute les constantes
    commands.iter().for_each(|x| {
        if x.0 == Commands::PrintStringConst {
            program.push_str(format!("  msg{} db '{}', 0xa\n", x.1[1], cut_string(&x.1[0])).as_str());
            program.push_str(format!("  msg{}len equ $ - msg{} \n", x.1[1], x.1[1]).as_str());
        }
    });

    program.push_str("section .text\n");

    program.push_str("dump:\n");
    program.push_str("    mov     r9, -3689348814741910323\n");
    program.push_str("    sub     rsp, 40\n");
    program.push_str("    mov     BYTE [rsp+31], 10\n");
    program.push_str("    lea     rcx, [rsp+30]\n");
    program.push_str(".L2:\n");
    program.push_str("    mov     rax, rdi\n");
    program.push_str("    lea     r8, [rsp+32]\n");
    program.push_str("    mul     r9\n");
    program.push_str("    mov     rax, rdi\n");
    program.push_str("    sub     r8, rcx\n");
    program.push_str("    shr     rdx, 3\n");
    program.push_str("    lea     rsi, [rdx+rdx*4]\n");
    program.push_str("    add     rsi, rsi\n");
    program.push_str("    sub     rax, rsi\n");
    program.push_str("    add     eax, 48\n");
    program.push_str("    mov     BYTE [rcx], al\n");
    program.push_str("    mov     rax, rdi\n");
    program.push_str("    mov     rdi, rdx\n");
    program.push_str("    mov     rdx, rcx\n");
    program.push_str("    sub     rcx, 1\n");
    program.push_str("    cmp     rax, 9\n");
    program.push_str("    ja      .L2\n");
    program.push_str("    lea     rax, [rsp+32]\n");
    program.push_str("    mov     edi, 1\n");
    program.push_str("    sub     rdx, rax\n");
    program.push_str("    xor     eax, eax\n");
    program.push_str("    lea     rsi, [rsp+32+rdx]\n");
    program.push_str("    mov     rdx, r8\n");
    program.push_str("    mov     rax, 1\n");
    program.push_str("    syscall\n");
    program.push_str("    add     rsp, 40\n");
    program.push_str("    ret\n");

    program.push_str("global _start\n");
    program.push_str("_start:\n");


    for command in commands {
        match (command.0, command.1) {
            (Commands::Push, args) => {
                program.push_str(format!(
                    "  ; push {}\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str(format!(
                    "  push {}\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::Dump, _) => {
                program.push_str("  ; dump stack\n");
                program.push_str("  pop rdi\n");
                program.push_str("  call dump\n\n");
            },
            (Commands::Add, _) => {
                program.push_str("  ; add\n");
                program.push_str("  pop rdi\n");
                program.push_str("  pop rax\n");
                program.push_str("  add rax, rdi\n");
                program.push_str("  push rax\n\n");
            },
            (Commands::Sub, _) => {
                program.push_str("  ; sub\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  sub rdi, rax\n");
                program.push_str("  push rdi\n\n");
            }
            (Commands::Dup, _) => {
                program.push_str("  ; dup\n");
                program.push_str("  pop rax\n");
                program.push_str("  push rax\n");
                program.push_str("  push rax\n\n");
            },
            (Commands::If, args) => {
                program.push_str("  ; if\n");
                program.push_str("  pop rdi\n");
                program.push_str("  cmp rdi, 0\n");
                program.push_str(format!(
                    "  je if_{}_end\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::EndIf, args) => {
                program.push_str(format!(
                    "  if_{}_end: ; end of the if\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::While, args) => {
                program.push_str(format!(
                    "  while_{}_start: ; start of the while clause\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str("  ; while\n");
                program.push_str("  pop rdi\n");
                program.push_str("  cmp rdi, 0\n");
                program.push_str(format!(
                    "  je while_{}_end\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::EndWhile, args) => {
                program.push_str("  ; jump to while\n");
                program.push_str(format!(
                    "  jmp while_{}_start\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str(format!(
                    "  while_{}_end ; end of the while\n\n", 
                    args[0].as_str()).as_str()
                );
            }
            (Commands::G, args) => {
                program.push_str(&generate_comparison_code("g", &args[0].as_str()));
            }
            (Commands::L, args) => {
                program.push_str(&generate_comparison_code("l", &args[0].as_str()));
            }
            (Commands::E, args) => {
                program.push_str(&generate_comparison_code("e", &args[0].as_str()));
            }
            (Commands::Ne, args) => {
                program.push_str(&generate_comparison_code("ne", &args[0].as_str()));
            }
            (Commands::Ge, args) => {
                program.push_str(&generate_comparison_code("ge", &args[0].as_str()));
            }
            (Commands::Le, args) => {
                program.push_str(&generate_comparison_code("le", &args[0].as_str()));
            },
            (Commands::PrintStringConst, args) => {
                program.push_str(format!(
                    "  ; print const message nb {}\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str("  mov rax, 1\n");
                program.push_str("  mov rdi, 1\n");
                program.push_str(format!(
                    "  mov rsi, msg{}\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str(format!(
                    "    mov rdx, msg{}len\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str("  syscall\n\n");
            }
        }
    }
    
    // exit
    program.push_str("  ; we end the program and return 0\n");
    program.push_str("  mov eax, 1\n");
    program.push_str("  mov ebx, 0\n");
    program.push_str("  int 0x80\n");
    file.write_all(program.as_bytes())?;
    Ok(())
}

fn tok_to_commands(tokens: Vec<String>) -> Vec<(Commands, Vec<String>)> {
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
            commands.push((Commands::Push, [token].to_vec()));
        }
        else if token == "." {
            commands.push((Commands::Dump, [].to_vec()));
        }
        else if token == "+" {
            commands.push((Commands::Add, [].to_vec()));
        }
        else if token == "-" {
            commands.push((Commands::Sub, [].to_vec()));
        }
        else if token == "dup" {
            commands.push((Commands::Dup, [].to_vec()));
        }
        else if token == "if" {
            commands.push((Commands::If, [unique_nb.to_string()].to_vec()));
            states.push_back((Commands::If, unique_nb));
            unique_nb += 1;
        }
        else if token == "while" {
            commands.push((Commands::While, [unique_nb.to_string()].to_vec()));
            states.push_back((Commands::While, unique_nb));
            unique_nb += 1;
        }
        else if token == ">" {
            commands.push((Commands::G, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if token == "<" {
            commands.push((Commands::L, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if token == "=" {
            commands.push((Commands::E, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if token == "!=" {
            commands.push((Commands::Ne, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if token == ">=" {
            commands.push((Commands::Ge, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if token == "<=" {
            commands.push((Commands::Le, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        else if is_string(&token) {
            commands.push((Commands::PrintStringConst, [token, format!("{}", mess_nb)].to_vec()));
            mess_nb += 1;
        }
        else {
            println!("Error : token: {}", token);
        }
    }
    commands
}

fn file_to_tok(file: &str) -> Vec<String> {
    let file = file.replace("\r", "");
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut inside_string = false;

    for c in file.chars() {
        if c == '\"' {
            inside_string = !inside_string; // Toggle the inside_string flag
            token.push(c);
        } else if inside_string || (c != ' ' && c != '\n') {
            token.push(c);
        } else if !token.is_empty() {
            tokens.push(token.clone());
            token.clear();
        }
    }

    if !token.is_empty() {
        tokens.push(token.clone());
    }

    tokens.retain(|x| x != "");
    tokens
}

fn generate_comparison_code(operation: &str, args: &str) -> String {
    let mut program = String::new();
    program.push_str(format!("  ; {}\n", operation).as_str());
    program.push_str("  pop rdi\n");
    program.push_str("  pop rax\n");
    program.push_str("  cmp rax, rdi\n");
    program.push_str(format!("  j{} {}_true_{}\n", operation, operation, args).as_str());
    program.push_str("  push 0\n");
    program.push_str(format!("  jmp {}_end_{}\n", operation, args).as_str());
    program.push_str(format!("  {}_true_{}:\n", operation, args).as_str());
    program.push_str("  push 1\n");
    program.push_str(format!("  {}_end_{}:\n\n", operation, args).as_str());

    program
}

fn is_string(token: &str) -> bool {
    if let Some(first_char) = token.chars().next() {
        if let Some(last_char) = token.chars().rev().next() {
            return first_char == '"' && last_char == '"';
        }
    }
    false
}

fn cut_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}