use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::Commands;

use crate::lexer;
use crate::parser;

pub fn make_asm(input_file: &str) -> std::io::Result<()> {
    let mut file = File::create("output/output.asm")?;
    let mut program = String::new();

    const MEMORY_SIZE: i32 = 1000;
    
    let mut input_file = File::open(input_file)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // we get tokens from file
    let tokens = lexer::file_to_tok(&contents.to_string());
    #[cfg(debug_assertions)]
    println!("tokens: {:?}", tokens);

    // we parse the macros
    let macros_tokens = parser::parse_macros(tokens);
    #[cfg(debug_assertions)]
    println!("after macros: {:?}", macros_tokens);

    // we get commands from tokens
    let commands = parser::tok_to_commands(macros_tokens);
    #[cfg(debug_assertions)]
    println!("commands: {:?}", commands);

    program.push_str("section .data\n");
    program.push_str("  ; constants\n");
    program.push_str("  NULL            equ 0\n");
    program.push_str("  EXIT_SUCCESS    equ 0\n");
    program.push_str("  EXIT_FAIL       equ 1\n");
    program.push_str("  SYS_exit        equ 60\n");
    program.push_str("  SYS_read        equ 0\n");
    program.push_str("  SYS_write       equ 1\n");
    program.push_str("  STD_in          equ 0\n");
    program.push_str("  STD_out         equ 1\n");
    program.push_str("  STD_err         equ 2\n\n");

    // on ajoute les constantes de texte
    commands.iter().for_each(|x| {
        if x.0 == Commands::PrintStringConst {
            program.push_str(format!("  msg{} db '{}', 0xa\n", x.1[1], parser::cut_string(&x.1[0])).as_str());
            program.push_str(format!("  msg{}len equ $ - msg{} \n", x.1[1], x.1[1]).as_str());
        }
    });

    program.push_str("section .bss\n");
    program.push_str(format!(
        "  mem: resb {}\n", 
        MEMORY_SIZE
    ).as_str());
    program.push_str("  \n");

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
                program.push_str("  mov rdi, STD_out\n");
                program.push_str(format!(
                    "  mov rsi, msg{}\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str(format!(
                    "  mov rdx, msg{}len\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str("  syscall\n\n");
            },
            #[allow(unreachable_code)] // pour que la macro marche
            (Commands::Syscall, args) => {
                program.push_str(format!(
                    "  ; syscall with {} args\n", 
                    args[0].as_str()).as_str()
                );
                let mut syscall_args: Vec<&str> = Vec::new();
                match_fallthrough!(args[0].as_str(), {
                    "6" => {syscall_args.push("  pop r9");},
                    "5" => {syscall_args.push("  pop r8");},
                    "4" => {syscall_args.push("  pop r10");},
                    "3" => {syscall_args.push("  pop rdx");},
                    "2" => {syscall_args.push("  pop rsi");},
                    "1" => {syscall_args.push("  pop rdi");},
                    "0" => {syscall_args.push("  pop rax");break;},
                    _ => { panic!("Unreachable"); },
                });
                for str in syscall_args.iter().rev() {
                    program.push_str(str);
                    program.push_str("\n");
                }
                program.push_str("  syscall\n\n");
            },
            (Commands::Mul, _) => {
                program.push_str("  ; mul\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  mul rdi\n");
                program.push_str("  push rax\n\n");
            },
            (Commands::Mem, _) => {
                program.push_str("  ; mem\n");
                program.push_str("  push mem\n\n");
            },
            (Commands::Read8, _) => {
                program.push_str("  ; read8\n");
                program.push_str("  pop rax\n");
                program.push_str("  xor rbx, rbx ; clean rxb\n");
                program.push_str("  mov bl, BYTE [rax]\n");
                program.push_str("  push rbx\n\n");
            },
            (Commands::Write8, _) => {
                program.push_str("  ; write8\n");
                program.push_str("  pop rbx\n");
                program.push_str("  pop rax\n");
                program.push_str("  mov [rax], bl\n\n");
            },
            (Commands::Swap, _) => {
                program.push_str("  ; swap\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  push rax\n");
                program.push_str("  push rdi\n\n");
            }
        }
    }
    
    // exit
    program.push_str("  ; we end the program and return 0\n");
    program.push_str("  mov rax, SYS_exit\n");
    program.push_str("  mov rdi, EXIT_SUCCESS\n");
    program.push_str("  syscall\n");
    file.write_all(program.as_bytes())?;
    Ok(())
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