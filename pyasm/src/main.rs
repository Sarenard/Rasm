use std::collections::VecDeque;
use std::process::Command;
use std::io::prelude::*;
use std::fs::File;

macro_rules! command_enum {
    ($($variant:ident),*) => {
        #[derive(Debug)]
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
    Gt
    // Lt,
    // Eq,
    // Neq,
    // Gte,
    // Lte,
);

fn main() -> std::io::Result<()> {

    make_asm().expect("failed to make asm");

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

fn make_asm() -> std::io::Result<()> {
    let mut file = File::create("output/output.asm")?;
    let mut program = String::new();
    // start
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

    let tokens = file_to_tok(include_str!("../input/input.pyasm"));
    println!("tokens: {:?}", tokens);
    let commands = tok_to_commands(tokens);
    println!("commands: {:?}", commands);

    for command in commands {
        match (command.0, command.1) {
            (Commands::Push, args) => {
                program.push_str("  ; push ");
                program.push_str(args[0].as_str());
                program.push_str("\n  push ");
                program.push_str(args[0].as_str());
                program.push_str("\n\n");
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
                program.push_str("  je if_");
                program.push_str(args[0].as_str());
                program.push_str("_end\n\n");
            },
            (Commands::EndIf, args) => {
                program.push_str("  if_");
                program.push_str(args[0].as_str());
                program.push_str("_end: ; end of the if\n\n");
            },
            (Commands::While, args) => {
                program.push_str("while_");
                program.push_str(args[0].as_str());
                program.push_str("_start: ; start of the while clause\n");
                program.push_str("  ; while\n");
                program.push_str("  pop rdi\n");
                program.push_str("  cmp rdi, 0\n");
                program.push_str("  je while_");
                program.push_str(args[0].as_str());
                program.push_str("_end\n\n");
            },
            (Commands::EndWhile, args) => {
                program.push_str("  ; jump to while\n");
                program.push_str("  jmp while_");
                program.push_str(args[0].as_str());
                program.push_str("_start\n");
                program.push_str("while_");
                program.push_str(args[0].as_str());
                program.push_str("_end: ; end of the while clause\n\n");
            }
            (Commands::Gt, args) => {
                program.push_str("  ; gt\n");
                program.push_str("  pop rdi\n");
                program.push_str("  pop rax\n");
                program.push_str("  cmp rax, rdi\n");
                program.push_str("  jg gt_true_");
                program.push_str(args[0].as_str());
                program.push_str("\n");
                program.push_str("  push 0\n");
                program.push_str("  jmp gt_end_");
                program.push_str(args[0].as_str());
                program.push_str("\n");
                program.push_str("gt_true_");
                program.push_str(args[0].as_str());
                program.push_str(":\n");
                program.push_str("  push 1\n");
                program.push_str("gt_end_");
                program.push_str(args[0].as_str());
                program.push_str(":\n\n");
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
            commands.push((Commands::Gt, [unique_nb.to_string()].to_vec()));
            unique_nb += 1;
        }
        // else if token == "<" {
        //     commands.push((Commands::Lt, [].to_vec()));
        // }
        // else if token == "==" {
        //     commands.push((Commands::Eq, [].to_vec()));
        // }
        // else if token == "!=" {
        //     commands.push((Commands::Neq, [].to_vec()));
        // }
        // else if token == ">=" {
        //     commands.push((Commands::Gte, [].to_vec()));
        // }
        // else if token == "<=" {
        //     commands.push((Commands::Lte, [].to_vec()));
        // }
        else {
            println!("Error : token: {}", token);
        }
    }
    commands
}

fn file_to_tok(file: &str) -> Vec<String> {
    // replace \r from the file with nothing
    let file = file.replace("\r", "");
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    for c in file.chars() {
        if c == ' ' || c == '\n' {
            tokens.push(token.clone());
            token.clear();
        } else {
            token.push(c);
        }
    }
    tokens.push(token.clone());
    // remove empty tokens
    tokens.retain(|x| x != "");
    tokens
}