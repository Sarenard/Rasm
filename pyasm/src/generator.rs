use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::Commands;
use crate::parser;

use crate::instructions::Settings;
use crate::instructions::Setting;

pub fn make_asm(commands: Vec<(Commands, Vec<String>)>, settings: Settings) -> std::io::Result<()> {
    let mut file = File::create("output/output.asm")?;
    let mut program = String::new();

    let memory_size: u64 = settings.get_value(Setting::MemorySize);
    let function_depth_limit: u64 = settings.get_value(Setting::FunctionDepthLimit);

    let functions: HashMap<String, String> = commands.iter()
    .filter_map(|x| {
        if x.0 == Commands::Func {
            Some((x.1[1].clone(), x.1[0].clone()))
        } else {
            None
        }
    })
    .collect();

    #[cfg(debug_assertions)]
    println!("functions: {:?}", functions);

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

    program.push_str("  REC_OVERFLOW_MSG db 'Recursion overflow', 10, '', 0\n");
    program.push_str("  REC_OVERFLOW_MSG_LEN equ $ - REC_OVERFLOW_MSG\n\n");

    // on ajoute les constantes de texte
    commands.iter().for_each(|x| {
        if x.0 == Commands::PrintStringConst {
            program.push_str(format!("  msg{} db '{}', 0\n", x.1[1], (parser::cut_string(&x.1[0]))).replace("\\n", "', 10, '").as_str());
            program.push_str(format!("  msg{}len equ $ - msg{} \n", x.1[1], x.1[1]).as_str());
        }
    });

    program.push_str("section .bss\n");
    program.push_str(format!(
        "  mem: resb {}\n", 
        memory_size
    ).as_str());
    program.push_str(format!(
        "  func_space: resb {}\n", 
        function_depth_limit * 8
    ).as_str());
    program.push_str("\n");

    program.push_str("section .text\n");

    // TODO : remove this hardcoded func
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
    program.push_str("    ret\n\n");

    program.push_str("global _start\n");
    program.push_str("_start:\n\n");

    let mut if_stack: Vec<(String, bool)> = vec![];

    // TODO : dont compile not used functions
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
                program.push_str("  pop rdi\n");
                program.push_str("  pop rax\n");
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
                if_stack.push((args[0].clone(), false));
                program.push_str("  ; if\n");
                program.push_str("  pop rdi\n");
                program.push_str("  cmp rdi, 0\n");
                program.push_str(format!(
                    "  je ifelse_{}\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::EndIf, args) => {
                let else_was_used = if_stack.pop().unwrap().1;
                if !else_was_used {
                    program.push_str(format!(
                        "  ifelse_{}: ; end of the if\n", 
                        args[0].as_str()).as_str()
                    );
                }
                program.push_str(format!(
                    "  if_end_{}: ; end of the if\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::Else, _args) => {
                let mut if_nb = if_stack.pop().unwrap();
                program.push_str(format!(
                    "  jmp if_end_{}\n", 
                    if_nb.0.as_str()).as_str()
                );
                program.push_str(format!(
                    "  ifelse_{}:\n\n", 
                    if_nb.0.as_str()).as_str()
                );
                if_nb.1 = true;
                if_stack.push(if_nb);
            }
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
                    "  ; put the *msg {} and the len on the stack\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str(format!(
                    "  mov rax, msg{}len\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str("  push rax\n\n");
                program.push_str(format!(
                    "  mov rax, msg{}\n", 
                    args[1].as_str()).as_str()
                );
                program.push_str("  push rax\n\n");
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
                program.push_str("  syscall\n");
                program.push_str("  push rax\n\n");
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
                program.push_str("  mov rdi, mem\n");
                program.push_str("  push rdi\n\n");
            },
            (Commands::Read, args) => {
                program.push_str(format!(
                    "  ; read{}\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str("  pop rax\n");
                program.push_str("  xor rbx, rbx ; clean rxb\n");
                match args[0].as_str() {
                    "8"  => {program.push_str("  mov bl, BYTE [rax]\n");},
                    "16" => {program.push_str("  mov bx, WORD [rax]\n");},
                    "32" => {program.push_str("  mov ebx, DWORD [rax]\n");},
                    "64" => {program.push_str("  mov rbx, QWORD [rax]\n");},
                    _ => { panic!("Wrong size for read"); },
                }
                program.push_str("  push rbx\n\n");
            },
            (Commands::Write, args) => {
                program.push_str(format!(
                    "  ; write{}\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str("  pop rbx\n");
                program.push_str("  pop rax\n");
                match args[0].as_str() {
                    "8"  => {program.push_str("  mov BYTE [rax], bl\n\n");},
                    "16" => {program.push_str("  mov WORD [rax], bx\n\n");},
                    "32" => {program.push_str("  mov DWORD [rax], ebx\n\n");},
                    "64" => {program.push_str("  mov QWORD [rax], rbx\n\n");},
                    _ => { panic!("Wrong size for write"); },
                }
            },
            (Commands::Swap, _) => {
                program.push_str("  ; swap\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  push rax\n");
                program.push_str("  push rdi\n\n");
            },
            (Commands::Drop, _) => {
                program.push_str("  ; drop\n");
                program.push_str("  pop rax\n\n");
            },
            (Commands::Over, _) => {
                program.push_str("  ; over\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  push rdi\n");
                program.push_str("  push rax\n");
                program.push_str("  push rdi\n\n");
            },
            (Commands::Rot, _) => {
                program.push_str("  ; rot\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rdi\n");
                program.push_str("  pop rbx\n");
                program.push_str("  push rax\n");
                program.push_str("  push rbx\n");
                program.push_str("  push rdi\n\n");
            },
            (Commands::Func, args) => {
                program.push_str(format!(
                    "  ; function definition : {} \n", 
                    args[1].as_str()).as_str()
                );
                program.push_str(format!(
                    "  jmp fn_{}_end\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str(format!(
                    "fn_{}_start:\n", 
                    args[0].as_str()).as_str()
                );
                program.push_str("  ; on check si la memoire est remplie\n");
                program.push_str("  xor rax, rax\n");
                program.push_str("  mov al, byte [func_space]\n");
                program.push_str("  cmp al, 10\n");
                program.push_str("  je RECURSION_LIMIT\n");
                program.push_str("  ; on se décale vers la droite\n");
                program.push_str("  xor rax, rax\n");
                program.push_str("  mov al, byte [func_space]\n");
                program.push_str("  mov rbx, 8\n");
                program.push_str("  mul rbx\n");
                program.push_str("  add rax, 1\n");
                program.push_str("  ; valeur de l'adresse de retour\n");
                program.push_str("  pop rbx\n");
                program.push_str("  mov [func_space+rax], rbx\n");
                program.push_str("  ; on incrémente [func_space]\n");
                program.push_str("  mov al, [func_space]\n");
                program.push_str("  add al, 1\n");
                program.push_str("  mov [func_space], al\n\n");
            },
            (Commands::EndFunc, args) => {
                program.push_str("  ; remove 1 from [func_space]\n");
                program.push_str("  mov al, [func_space]\n");
                program.push_str("  sub al, 1\n");
                program.push_str("  mov [func_space], al\n");
                program.push_str("  ; compute of return adress\n");
                program.push_str("  xor rax, rax\n");
                program.push_str("  mov al, byte [func_space]\n");
                program.push_str("  mov rbx, 8\n");
                program.push_str("  mul rbx\n");
                program.push_str("  add rax, 1\n");
                program.push_str("  ; return \n");
                program.push_str("  mov rax, [func_space+rax]\n");
                program.push_str("  jmp rax\n\n");
                program.push_str(format!(
                    "fn_{}_end:\n\n", 
                    args[0].as_str()).as_str()
                );
            },
            (Commands::Unknown, args) => {
                // si c'est dans la hashmap functions
                if functions.contains_key(args[0].as_str()) {
                    program.push_str(format!(
                        "  ; call function : {} \n", 
                        args[0].as_str()).as_str()
                    );
                    program.push_str(format!(
                        "  call fn_{}_start\n\n", 
                        functions[args[0].as_str()]).as_str()
                    );
                } else {
                    panic!("Unknown command : {}", args[0].as_str());
                }
            },
            (Commands::Div, _) => {
                program.push_str("  ; divmod\n");
                program.push_str("  xor rdx, rdx\n");
                program.push_str("  pop rax\n");
                program.push_str("  pop rcx\n");
                program.push_str("  div rcx\n");
                program.push_str("  push rdx\n");
                program.push_str("  push rax\n\n");
            },
        }
    }
    
    // exit
    program.push_str("  ; we end the program and return 0\n");
    program.push_str("  mov rax, SYS_exit\n");
    program.push_str("  mov rdi, EXIT_SUCCESS\n");
    program.push_str("  syscall\n\n");

    program.push_str("RECURSION_LIMIT:\n");
    program.push_str("  ; print error msg\n");
    program.push_str("  mov rax, 1\n");
    program.push_str("  mov rdi, 1\n");
    program.push_str("  mov rsi, REC_OVERFLOW_MSG\n");
    program.push_str("  mov rdx, REC_OVERFLOW_MSG_LEN\n");
    program.push_str("  syscall\n\n");
    program.push_str("  ; exit\n");
    program.push_str("  pop rax\n");
    program.push_str("  mov rax, SYS_exit\n");
    program.push_str("  mov rdi, EXIT_FAIL\n");
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