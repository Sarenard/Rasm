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

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Setting {
    MemorySize,
    FunctionDepthLimit,
}

pub type TSetting = (Setting, u64);

pub struct Settings {
    pub settings_array: Vec<TSetting>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            settings_array: Vec::new(),
        }
    }

    pub fn add_setting(&mut self, setting: Setting, value: u64) {
        // if already exists, panic
        if self.settings_array.iter().any(|(s, _)| s == &setting) {
            panic!("Setting {:?} already exists", setting);
        }
        self.settings_array.push((setting, value));
    }

    pub fn get_value(&self, setting: Setting) -> u64 {
        // get type of setting without value
        for (s, v) in &self.settings_array {
            if s == &setting {
                return *v;
            }
        }
        panic!("Setting {:?} not found", setting);
    }

}