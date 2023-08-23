use crate::Commands;

pub fn simulate(commands: Vec<(Commands, Vec<String>)>) -> u8 {

    let mut stack: Vec<u8> = vec![];
    let mut index: usize = 0;

    // TODO : finish simulating mode
    while index < commands.len() {
        let command = commands[index].0.clone();
        let args: Vec<String> = commands[index].1.clone();
        match (command, args) {
            (Commands::Push, args) => {
                stack.push(args[0].parse::<u8>().unwrap());
            },
            (Commands::Dump, _) => {
                println!("{:?}", stack.pop().unwrap());
            },
            (Commands::Add, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            (Commands::Sub, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b);
            }
            (Commands::Dup, _) => {
                let last = stack.pop().unwrap();
                stack.push(last);
                stack.push(last);
            },
            (Commands::If, _) => {

            },
            (Commands::EndIf, _) => {

            },
            (Commands::Else, _) => {

            }
            (Commands::While, _) => {

            },
            (Commands::EndWhile, _) => {

            }
            (Commands::G, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a > b) as u8);
            }
            (Commands::L, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a < b) as u8);
            }
            (Commands::E, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a == b) as u8);
            }
            (Commands::Ne, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a != b) as u8);
            }
            (Commands::Ge, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a >= b) as u8);
            }
            (Commands::Le, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a <= b) as u8);
            },
            (Commands::PrintStringConst, _) => {

            },
            (Commands::Syscall, _) => {

            },
            (Commands::Mul, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            (Commands::Mem, _) => {

            },
            (Commands::Read, _) => {

            },
            (Commands::Write, _) => {

            },
            (Commands::Swap, _) => {

            },
            (Commands::Drop, _) => {

            },
            (Commands::Over, _) => {

            },
            (Commands::Rot, _) => {

            },
            (Commands::Func, _) => {

            },
            (Commands::EndFunc, _) => {

            },
            (Commands::Unknown, _) => {
                
            },
            (Commands::Div, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                // divmod
                stack.push(a / b);
                stack.push(a % b);
            },
        }
        index += 1;
    }

    return 0;
}