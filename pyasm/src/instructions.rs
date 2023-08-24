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