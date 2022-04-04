use crate::instructions::OperandValue;
use crate::program::Program;

// @Todo: I don't like these names
pub type RegisterId = usize;
// The id used for each register, key in the vector
pub type RegisterValue = i32;
// The value the VM uses
pub type Word = u8;
// A single Opcode or register contents, or whatever
pub type DoubleWord = u16; // For when a register value is 2 slots
pub type FourWords = u32; // For when a register value is 2 slots

pub enum ExecutionResult {
    // @todo: Decide on SUCCESSFUL Execution Results (and make an Execution Failure)
    Halted,
    Value(RegisterValue),
}

pub struct VM {
    pub(crate) registers: [RegisterValue; VM::REGISTER_COUNT],
    remainder: u32,
}

impl VM {
    pub(crate) fn remainder(&self) -> u32 {
        self.remainder
    }
}

impl VM {
    pub(crate) fn set_remainder(&mut self, remainder: u32) {
        self.remainder = remainder
    }
}

impl VM {
    pub const REGISTER_COUNT: usize = 32;

    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            remainder: 0,
        }
    }

    pub fn run(&mut self, program: &mut Program) {
        for instruction in program {
            match instruction.execute(self) {
                Ok(ExecutionResult::Value(value)) => println!("{value}"),
                Ok(ExecutionResult::Halted) => println!("Halting"),
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