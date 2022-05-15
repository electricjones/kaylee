use std;
use std::io;
use std::io::Write;

use nom::IResult;

use crate::asm::assembler::Assembler;
use crate::asm::parser::{line, parse_asm};
use crate::instructions::decode_next_instruction;
use crate::program::Program;
use crate::shared::parse_hex;
use crate::vm::Kaylee;

/// Core structure for the REPL for the Assembler
pub struct Repl {
    command_buffer: Vec<String>,
    vm: Kaylee,
}

impl Repl {
    /// Creates and returns a new asm REPL
    pub fn new() -> Repl {
        Repl {
            vm: Kaylee::new(),
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
                    match parse_asm(buffer) {
                        Ok(parsed) => {
                            let assembler = Assembler::new();
                            let results = assembler.assemble_parsed_asm(parsed.1);
                            match results {
                                Ok(bytes) => {
                                    let _ = &program.extend(bytes);
                                    self.vm.run_next(&program)
                                }
                                Err(_e) => {
                                    println!("Invalid string bytecode");
                                }
                            }
                        }
                        Err(error) => {
                            println!("{:?}", error);
                        }
                    }
                }
            }
        }
    }
}