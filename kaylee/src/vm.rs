use crate::instructions::{decode_next_instruction, Instruction};
use crate::program::{Program, ProgramIndex};

// The id used for each register, key in the vector
pub type RegisterId = usize;

// The value the VM uses
pub type RegisterValue = i32;

// A single Opcode or register contents, or whatever
pub type Byte = u8;

// For when a register value is 2 slots
pub type HalfWord = u16;
pub type Word = u32;
pub type DoubleWord = u32;

pub enum ExecutionResult {
    Halted,
    NoAction,
    Value(RegisterValue),
    Jumped(ProgramIndex),
    Equality(bool),
}

pub enum ExecutionError {
    Unknown(String),
}

pub struct Kaylee {
    registers: [RegisterValue; Kaylee::REGISTER_COUNT],
    program_counter: RegisterId,
    remainder: u32,
    halted: bool,
}

impl Kaylee {
    pub const REGISTER_COUNT: usize = 32;

    pub fn new() -> Self {
        Kaylee {
            registers: [0; Kaylee::REGISTER_COUNT],
            remainder: 0,
            program_counter: 0,
            halted: false,
        }
    }

    /// This will run until one of the following conditions is met
    /// 1. The Program reaches completes its final instruction
    /// 2. The VM `halt` flag is set, which will complete the current instruction and then halt
    pub fn run(&mut self, program: Program) {
        while let Some(result) = decode_next_instruction(&program, &mut self.program_counter) {
            match result {
                Ok(instruction) => { self.execute_instruction(instruction) }
                Err(_error) => { panic!("Error decoding instruction") }
            }

            if self.halted {
                break;
                // @todo: graceful shutdown of the machine/process
            }
        }
    }

    pub fn run_next(&mut self, program: &Program) {
        match decode_next_instruction(program, &mut self.program_counter) {
            Some(Ok(instruction)) => self.execute_instruction(instruction),
            None => println!("Execution Finished"),
            Some(Err(_error)) => panic!("received an error"),
        };
    }

    fn execute_instruction(&mut self, instruction: Box<dyn Instruction>) {
        instruction.execute(self);

        // I should probably do something with these results, or pass them back
        // match instruction.execute(self) {
        //     Ok(ExecutionResult::Value(value)) => println!("{value}"),
        //     Ok(ExecutionResult::Halted) => println!("Halting"),
        //     Ok(ExecutionResult::Jumped(index)) => println!("Jumped to {index}"),
        //     Ok(ExecutionResult::Equality(flag)) => println!("Jumped to {flag}"),
        //     Ok(ExecutionResult::NoAction) => println!("No Action"),
        //     Err(_) => panic!("Error")
        // }
    }

    pub(crate) fn register(&self, register: RegisterId) -> Result<RegisterValue, ()> {
        if register > Kaylee::REGISTER_COUNT - 1 {
            return Err(());
        }

        Ok(*&self.registers[register].clone())
    }

    pub(crate) fn all_registers(&self) -> [RegisterValue; Kaylee::REGISTER_COUNT] {
        *&self.registers.clone()
    }

    pub(crate) fn set_register(&mut self, register: RegisterId, value: RegisterValue) -> Result<(), ()> {
        if register > Kaylee::REGISTER_COUNT - 1 {
            return Err(());
        }

        self.registers[register] = value;
        Ok(())
    }

    pub(crate) fn halt(&mut self) {
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
        self.program_counter = index
    }
}

#[cfg(test)]
mod tests {

}
