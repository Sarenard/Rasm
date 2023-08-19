#[macro_use] extern crate fallthrough;
use std::{env, process::Command};

mod lexer;
mod parser;
mod generator;

macro_rules! command_enum {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        #[derive(PartialEq)]
        pub enum Commands {
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
    PrintStringConst, // temporary
    Syscall,
    Mul,
    Mem,
    Read,
    Write,
    Swap,
    Drop,
    Over,
    Rot,
    Func,
    EndFunc,
    Unknown,
    Div
);

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: pyasm -f <input file>");
        return Ok(());
    }

    let input_file = &args[2];

    generator::make_asm(input_file).expect("failed to make asm");

    let nasm_output = Command::new("nasm")
        .args(["-f", "elf64", "output/output.asm", "-o", "output/output.o"])
        .output()
        .expect("failed to execute process");
    if !nasm_output.status.success() {
        println!("nasm output: {:?}", nasm_output);
        return Ok(());
    }

    let ld_output = Command::new("ld")
        .args(["-m", "elf_x86_64", "output/output.o", "-o", "output/output"])
        .output()
        .expect("failed to execute process");
    if !ld_output.status.success() {
        println!("ld output: {:?}", ld_output);
        return Ok(());
    }

    Ok(())
}
