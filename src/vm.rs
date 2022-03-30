use std::fmt::Error;

use crate::instructions::Instruction;
use crate::vm::instructions::Opcode;

mod instructions;

pub enum ExecutionResult {
    Halted,
    Value(i32),
    // @todo: Do I need this?
}

pub struct Program {
    counter: usize,
    instructions: Vec<u8>,
}


impl Program {
    pub fn new(instructions: Vec<u8>) -> Self {
        Program {
            counter: 0,
            instructions,
        }
    }
}

impl Iterator for Program {
    type Item = Box<dyn Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.counter >= &self.instructions.len() {
            return None;
        }

        Some(crate::instructions::decode_next_instruction(&mut self.instructions, &mut self.counter).unwrap())
        // *&mut self.counter += 1;
        // Some(instruction)
    }
}

pub struct VM {
    program: Program,
    pub(crate) registers: [i32; 32],
    counter: usize,
    pub instructions: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        VM {
            program: Program::new(vec![]),
            registers: [0; 32],
            counter: 0,
            instructions: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self, program: &mut Program) {
        for instruction in program {
            match instruction.execute(self) {
                Ok(ExecutionResult::Value(value)) => println!("{value}"),
                Ok(ExecutionResult::Halted) => println!("Halting"),
                Err(error) => panic!("Error")
            }
        }

        // let mut is_done = false;
        // while !is_done {
        //     is_done = self.execute_instruction();
        // }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.counter >= self.instructions.len() {
            return true;
        }

        true
    }

    pub(crate) fn set_register(&mut self, register: usize, value: i32) {
        self.registers[register] = value;
    }

    // @todo: this shouldn't be mutable
    // @todo: next_instruction() should be mutable
    fn decode_next_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.instructions[self.counter]);
        self.counter += 1;
        opcode
    }
    pub fn next_8_bits(&mut self) -> u8 {
        let result = self.instructions[self.counter];
        self.counter += 1;
        result
    }
    pub fn next_16_bits(&mut self) -> u16 {
        let result = ((self.instructions[self.counter] as u16) << 8) | self.instructions[self.counter + 1] as u16;
        self.counter += 2;
        result
    }


    pub fn halt(&mut self) {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let mut program = Program::new(vec![
            // 0,           // HALT
            1, 4, 1, 244, // LOAD $4 #500
            1, 6, 0, 12,  // LOAD $6 #12
        ]);

        // for instruction in program {
        //     assert_eq!(instruction.name(), "Load".to_string());
        // }

        let mut vm = VM::new();
        vm.run(&mut program);
        assert_eq!(true, true);
    }

//     #[test]
//     fn test_create_vm() {
//         let test_vm = VM::new();
//         assert_eq!(test_vm.registers[0], 0)
//     }
//
//     #[test]
//     fn test_opcode_halt() {
//         let mut test_vm = VM::new();
//         let test_bytes = vec![0, 0, 0, 0];
//         test_vm.instructions = test_bytes;
//         test_vm.run();
//         assert_eq!(test_vm.counter, 1);
//     }
//
//     #[test]
//     fn test_opcode_illegal() {
//         let mut test_vm = VM::new();
//         let test_bytes = vec![200, 0, 0, 0];
//         test_vm.instructions = test_bytes;
//         test_vm.run();
//         assert_eq!(test_vm.counter, 1);
//     }
//
//     #[test]
//     fn test_load_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![1, 0, 1, 244];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 500);
//     }
//
//     #[test]
//     fn test_add() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 4, // LOAD $1 #4
//             1, 2, 0, 5, // LOAD $2 #5
//             2, 0, 1, 2, // ADD $0 $1 $2
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 9);
//     }
//
//     #[test]
//     fn test_subtract() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 10, // LOAD $1 #4
//             1, 2, 0, 6,  // LOAD $2 #5
//             3, 0, 1, 2,  // SUB $0 $1 $2
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 4);
//     }
//
//     #[test]
//     fn test_multiply() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 2, // LOAD $1 #2
//             1, 2, 0, 3,  // LOAD $2 #3
//             4, 0, 1, 2,  // MUL $0 $1 $2
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 6);
//     }
//
//     #[test]
//     fn test_divide_no_remainder() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 6, // LOAD $1 #2
//             1, 2, 0, 2,  // LOAD $2 #3
//             5, 0, 1, 2,  // DIV $0 $1 $2
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 3);
//         assert_eq!(test_vm.remainder, 0);
//     }
//
//     #[test]
//     fn test_divide_with_remainder() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 6, // LOAD $1 #2
//             1, 2, 0, 4,  // LOAD $2 #3
//             5, 0, 1, 2,  // DIV $0 $1 $2
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 1);
//         assert_eq!(test_vm.remainder, 2);
//     }
//
//     #[test]
//     fn test_math() {
//         let mut test_vm = VM::new();
//         test_vm.instructions = vec![
//             1, 1, 0, 6, // LOAD $1 #2
//             1, 2, 0, 4,  // LOAD $2 #3
//             2, 0, 1, 2,  // ADD $0 $1 $2
//             3, 3, 0, 2,  // SUB $3 $0 $2
//             4, 4, 1, 2,  // MUL
//             5, 5, 0, 1,  // DIV
//         ];
//         test_vm.run();
//         assert_eq!(test_vm.registers[0], 10);
//         assert_eq!(test_vm.registers[1], 6);
//         assert_eq!(test_vm.registers[2], 4);
//         assert_eq!(test_vm.registers[3], 6);
//         assert_eq!(test_vm.registers[4], 24);
//         assert_eq!(test_vm.registers[5], 1);
//     }
//
//     #[test]
//     fn test_jmp_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 1;
//         test_vm.instructions = vec![6, 0, 0, 0];
//         test_vm.run_once();
//         assert_eq!(test_vm.counter, 1);
//     }
//
//     #[test]
//     fn test_jmpf_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.instructions = vec![7, 0, 0, 0, 6, 0, 0, 0];
//         test_vm.run_once();
//         assert_eq!(test_vm.counter, 4);
//     }
//
//     #[test]
//     fn test_jmpb_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.instructions = vec![8, 0, 0, 0, 6, 0, 0, 0];
//         test_vm.run_once();
//         assert_eq!(test_vm.counter, 0);
//     }
//
//     #[test]
//     fn test_eq_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.registers[1] = 2;
//         test_vm.instructions = vec![9, 2, 0, 1];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 1);
//
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.registers[1] = 3;
//         test_vm.instructions = vec![9, 2, 0, 1];
//         test_vm.run();
//         assert_eq!(test_vm.registers[2], 0);
//     }
//
//     #[test]
//     fn test_jeq_opcode() {
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.registers[1] = 2;
//         test_vm.registers[2] = 10;
//         test_vm.instructions = vec![15, 2, 0, 1];
//         test_vm.run();
//         assert_eq!(test_vm.counter, 10);
//
//         let mut test_vm = VM::new();
//         test_vm.registers[0] = 2;
//         test_vm.registers[1] = 3;
//         test_vm.registers[2] = 10;
//         test_vm.instructions = vec![9, 2, 0, 1];
//         test_vm.run();
//         assert_eq!(test_vm.counter, 4);
//     }
}