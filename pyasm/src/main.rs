#[macro_use] extern crate fallthrough;
use std::process::Command;

use clap::{App, Arg};

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
    Else,
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

    let matches = App::new("pyasm")
        .about("A stack based language")
        .arg(
            Arg::with_name("input")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to run")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();

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
