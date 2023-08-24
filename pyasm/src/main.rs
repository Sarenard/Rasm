#[macro_use] extern crate fallthrough;
use std::{process::Command, str, fs::File, io::Read, collections::HashMap};

use clap::{App, Arg};

mod lexer;
mod parser;
mod generator;
mod simulator;

macro_rules! command_enum {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        #[derive(PartialEq)]
        #[derive(Clone)]
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

// TODO : debug mode
fn main() -> std::io::Result<()> {

    let matches = App::new("pyasm")
        .about("A stack based language")
        .arg(
            Arg::with_name("input")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to run")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("compile")
                .short('c')
                .long("compile")
                .value_name("COMPILES")
                .help("If specified, compiles the program into asm")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("run")
                .conflicts_with("simulate")
                .requires("compile")
                .short('r')
                .long("run")
                .value_name("RUN")
                .help("If specified, runs the program after the compilation")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("simulate")
                .conflicts_with("run")
                .short('s')
                .long("sim")
                .value_name("SIMULATES")
                .help("If specified, simulates the program")
                .takes_value(false)
        )
        .get_matches();

    let input_name = matches.value_of("input").unwrap();

    let mut input_file = File::open(input_name)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // we get tokens from file
    let tokens = lexer::file_to_tok(&contents.to_string());
    #[cfg(debug_assertions)]
    println!("tokens: {:?}", tokens);

    // we parse the includes
    let includes_tokens = parser::parse_includes(tokens);
    #[cfg(debug_assertions)]
    println!("after includes: {:?}", includes_tokens);

    // TODO : handle bools
    // we parse the macros
    let macros_tokens = parser::parse_macros(includes_tokens, HashMap::new());
    #[cfg(debug_assertions)]
    println!("after macros: {:?}", macros_tokens);

    // we get commands from tokens
    let commands: Vec<(Commands, Vec<String>)> = parser::tok_to_commands(macros_tokens);
    #[cfg(debug_assertions)]
    println!("commands: {:?}", commands);

    // TODO : type/stack simulation to check for errors
    
    // TODO : error system
    // TODO : memory allocator
    if matches.contains_id("compile") {
        generator::make_asm(commands).expect("failed to make asm");
    }
    else if matches.contains_id("simulate") {
        let error_code = simulator::simulate(commands);
        std::process::exit(error_code as i32);
    }

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

    // run the program as if it was a binary
    if matches.contains_id("run") {
        let output = Command::new("./output/output")
            .output()
            .expect("failed to execute process");
        print!("{}", str::from_utf8(&output.stdout).unwrap());
    }

    Ok(())
}
