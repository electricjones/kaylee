use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

use crate::shared::parse_hex;
use crate::vm::VM;

/// Core structure for the REPL for the Assembler
pub struct Repl {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl Repl {
    /// Creates and returns a new assembly REPL
    pub fn new() -> Repl {
        Repl {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the Kaylee REPL");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush standard out");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{command}");
                    }
                }
                ".program" => {
                    println!("Listing entire program instructions");
                    for instruction in &self.vm.instructions {
                        println!("{instruction}");
                    }
                    println!("End of instructions");
                }
                ".registers" => {
                    println!("Listing all registers and contents");
                    println!("{:#?}", self.vm.registers);
                    println!("End of register listing");
                }
                _ => {
                    let results = parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            self.vm.instructions.extend(bytes);
                        }
                        Err(_e) => {
                            println!("Invalid HEX string");
                        }
                    }

                    self.vm.run_once();
                }
            }
        }
    }
}