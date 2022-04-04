use crate::instructions::Instruction;

// @Todo: I don't like these names
pub type RegisterId = usize;
// The id used for each register, key in the vector
pub type RegisterValue = i32;
// The value the VM uses
pub type Word = u8;
// A single Opcode or register contents, or whatever
pub type DoubleWord = u16;
// For when a register value is 2 slots
pub type FourWords = u32; // For when a register value is 2 slots

pub enum ExecutionResult {
    // @todo: Decide on SUCCESSFUL Execution Results (and make an Execution Failure)
    Halted,
    Value(RegisterValue),
    Jumped(ProgramIndex),
    Equality(bool),
}

pub struct VM {
    pub(crate) registers: [RegisterValue; VM::REGISTER_COUNT],
    remainder: u32,
    program_counter: usize,
    halted: bool,
}

impl VM {
    pub const REGISTER_COUNT: usize = 32;

    pub fn new() -> Self {
        VM {
            registers: [0; VM::REGISTER_COUNT],
            remainder: 0,
            program_counter: 0,
            halted: false,
        }
    }

    fn next(&mut self, instructions: &Program) -> Option<Box<dyn Instruction>> {
        if self.program_counter >= instructions.len() || self.halted {
            return None;
        }

        Some(crate::instructions::decode_next_instruction(instructions, &mut self.program_counter).unwrap())
    }


    // @todo: I don't want the program to be mutable, except for the counter
    pub fn run(&mut self, program: Program) {
        while let Some(instruction) = self.next(&program) {
            match instruction.execute(self) {
                Ok(ExecutionResult::Value(value)) => println!("{value}"),
                Ok(ExecutionResult::Halted) => println!("Halting"),
                Ok(ExecutionResult::Jumped(index)) => println!("Jumped to {index}"),
                Ok(ExecutionResult::Equality(flag)) => println!("Jumped to {flag}"),
                Err(_) => panic!("Error")
            }
        }

        // let mut is_done = false;
        // while !is_done {
        //     is_done = self.execute_instruction();
        // }
    }

    // pub fn run_once(&mut self) {
    //     self.execute_instruction();
    // }

    pub(crate) fn register(&self, register: RegisterId) -> Result<RegisterValue, ()> {
        if register > VM::REGISTER_COUNT - 1 {
            return Err(());
        }

        Ok(*&self.registers[register].clone())
    }

    pub(crate) fn all_registers(&self) -> [RegisterValue; VM::REGISTER_COUNT] {
        *&self.registers.clone()
    }

    pub(crate) fn set_register(&mut self, register: RegisterId, value: RegisterValue) -> Result<(), ()> {
        if register > VM::REGISTER_COUNT - 1 {
            return Err(());
        }

        self.registers[register] = value;
        Ok(())
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub(crate) fn remainder(&self) -> u32 {
        self.remainder
    }

    pub(crate) fn set_remainder(&mut self, remainder: u32) {
        self.remainder = remainder
    }

    pub(crate) fn program_counter(&self) -> ProgramIndex {
        self.program_counter
    }

    pub(crate) fn set_program_counter(&mut self, index: ProgramIndex) {
        // @todo: No program checking here since VM doesn't actually have a program
        self.program_counter = index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let program = Program::from([
            // 0,           // HALT
            1, 4, 1, 244, // LOAD $4 #500
            1, 6, 0, 12,  // LOAD $6 #12
        ]);

        // for instruction in program {
        //     assert_eq!(instruction.name(), "Load".to_string());
        // }

        let mut vm = VM::new();
        vm.run(program);
        assert_eq!(true, true);
    }

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

pub type ProgramIndex = usize;
pub type Program = Vec<u8>;
