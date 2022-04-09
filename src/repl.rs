use std;
use std::io;
use std::io::Write;

use crate::instructions::decode_next_instruction;
use crate::shared::parse_hex;
// use std::num::ParseIntError;
use crate::vm::Program;
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

        let mut program = Program::new();

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
                    let mut pc: usize = 0;

                    while let Some(result) = decode_next_instruction(&program, &mut pc) {
                        match result {
                            Ok(instruction) => println!("{}", instruction.display()),
                            Err(_error) => panic!("received an error")
                        };
                    }

                    println!("End of instructions");
                }
                ".registers" => {
                    println!("Listing all registers and contents");
                    println!("{:#?}", self.vm.all_registers());
                    println!("End of register listing");
                }
                _ => {
                    let results = parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            &program.extend(bytes);
                        }
                        Err(_e) => {
                            println!("Invalid HEX string");
                        }
                    }

                    self.vm.run_next(&program);
                }
            }
        }
    }
}