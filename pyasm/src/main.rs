use std::fs::File;
use std::io::prelude::*;

use std::process::Command;

fn main() -> std::io::Result<()> {
    
    make_asm().expect("failed to make asm");

    let _nasm_output = Command::new("nasm")
        .args(["-f", "elf64", "output/output.asm", "-o", "output/output.o"])
        .output()
        .expect("failed to execute process");
    println!("nasm output: {:?}", _nasm_output);
    let _ld_output = Command::new("ld")
        .args(["-m", "elf_x86_64", "output/output.o", "-o", "output/output"])
        .output()
        .expect("failed to execute process");
    println!("ld output: {:?}", _ld_output);
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

    for token in tokens {
        // si c'est un nombre
        if token.chars().all(char::is_numeric) {
            program.push_str("  push ");
            program.push_str(token.as_str());
            program.push_str("\n");
        }
        else if token == "." {
            program.push_str("  pop rdi\n");
            program.push_str("  call dump\n");
        }
        else if token == "+" {
            program.push_str("  pop rdi\n");
            program.push_str("  pop rax\n");
            program.push_str("  add rax, rdi\n");
            program.push_str("  push rax\n");
        }
        else {
            println!("Error : token: {}", token);
        }
    }
    
    // exit
    program.push_str("  mov eax, 1\n");
    program.push_str("  mov ebx, 12\n");
    program.push_str("  int 0x80\n");
    file.write_all(program.as_bytes())?;
    Ok(())
}

fn file_to_tok(file: &str) -> Vec<String> {
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
    tokens
}