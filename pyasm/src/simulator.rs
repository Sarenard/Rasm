use crate::Commands;
use crate::instructions::Settings;
use crate::instructions::Setting;

pub fn simulate(commands: Vec<(Commands, Vec<String>)>, settings: Settings) -> u8 {

    let _memory_size: u64 = settings.get_value(Setting::MemorySize);
    let _function_depth_limit: u64 = settings.get_value(Setting::FunctionDepthLimit);

    let mut stack: Vec<u64> = vec![];
    let mut index: usize = 0;

    let mut skip_stack: Vec<(bool, String)> = vec![];

    // TODO : finish simulating mode
    while index < commands.len() {
        let command = commands[index].0.clone();
        let args: Vec<String> = commands[index].1.clone();
        if skip_stack.last().unwrap_or(&(false, "".to_string())).0 && !matches!(command, Commands::Else | Commands::EndIf)  {
            index += 1;
            continue;
        }
        #[cfg(debug_assertions)]
        println!("{} {:?}", command, skip_stack);
        match (command, args) {
            (Commands::Push, args) => {
                stack.push(args[0].parse::<u64>().unwrap());
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
            (Commands::If, args) => {
                skip_stack.push((stack.pop().unwrap() == 0, args[0].clone()));
            },
            (Commands::EndIf, args) => {
                let (should_run, number) = skip_stack.pop().unwrap();
                if number != args[0] {
                    skip_stack.push((should_run, number));
                    index += 1;
                    continue;
                }
            },
            (Commands::Else, args) => {
                let (should_run, number) = skip_stack.pop().unwrap();
                if number != args[0] {
                    skip_stack.push((should_run, number));
                    index += 1;
                    continue;
                }
                skip_stack.push((!should_run, number));
            }
            (Commands::While, _) => {
                todo!();
            },
            (Commands::EndWhile, _) => {
                todo!();
            }
            (Commands::G, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a < b) as u64);
            }
            (Commands::L, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a > b) as u64);
            }
            (Commands::E, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a == b) as u64);
            }
            (Commands::Ne, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a != b) as u64);
            }
            (Commands::Ge, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a <= b) as u64);
            }
            (Commands::Le, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push((a >= b) as u64);
            },
            (Commands::PrintStringConst, _) => {
                todo!();
            },
            (Commands::Syscall, _) => {
                todo!();
            },
            (Commands::Mul, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            (Commands::Mem, _) => {
                todo!();
            },
            (Commands::Read, _) => {
                todo!();
            },
            (Commands::Write, _) => {
                todo!();
            },
            (Commands::Swap, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a);
                stack.push(b);
            },
            (Commands::Drop, _) => {
                let _ = stack.pop().unwrap();
            },
            (Commands::Over, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
                stack.push(b);
            },
            (Commands::Rot, _) => {
                todo!();
            },
            (Commands::Func, _) => {
                todo!();
            },
            (Commands::EndFunc, _) => {
                todo!();
            },
            (Commands::Unknown, _) => {
                todo!();
            },
            (Commands::Div, _) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                // divmod
                stack.push(a % b);
                stack.push(a / b);
            },
        }
        index += 1;
    }

    return 0;
}