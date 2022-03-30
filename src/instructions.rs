use std::fmt::Error;

use crate::instructions::data::Load;
use crate::instructions::system::Halt;
use crate::vm::{ExecutionResult, VM};

mod system;
mod data;

type OperandMap = [usize; 3];

type OperandValues = [OperandValue; 3];

pub enum OperandValue {
    u8(u8),
    u16(u16),
    None,
}

#[derive(Debug)]
pub enum InstructionDecodeError {
    InvalidValueSize,
    IllegalOpcode,
}

// @todo: Note that I went with the unconventional idea of variable instruction length
// @todo: Turn all the statics to methods
// @todo: Test the LOAD instruction
// @todo: Refactor the rest to use new instructions
// @todo: cache decoded instructions to save on performance when jumping
pub fn decode_next_instruction(instructions: &mut Vec<u8>, program_counter: &mut usize) -> Result<Box<dyn Instruction>, InstructionDecodeError> {
    let opcode: u8 = instructions[*program_counter];
    *program_counter += 1;

    match opcode {
        Halt::OPCODE => build::<Halt>(instructions, program_counter),
        Load::OPCODE => build::<Load>(instructions, program_counter),
        _ => {
            return Err(InstructionDecodeError::IllegalOpcode);
        }
    }
}


pub fn build<T: 'static + Instruction>(instructions: &mut Vec<u8>, program_counter: &mut usize) -> Result<Box<dyn Instruction>, InstructionDecodeError> {
    Ok(
        Box::new(
            T::new(
                consume_and_parse_values(
                    T::operand_map(),
                    instructions,
                    program_counter,
                )?
            )
        )
    )
}

pub fn consume_and_parse_values(operand_map: OperandMap, instructions: &mut Vec<u8>, program_counter: &mut usize) -> Result<OperandValues, InstructionDecodeError> {
    let mut operand_values: OperandValues = [OperandValue::None, OperandValue::None, OperandValue::None];

    for (index, bytes) in operand_map.iter().enumerate() {
        match bytes {
            0 => {
                operand_values[index] = OperandValue::None;
            }
            1 => {
                operand_values[index] = OperandValue::u8(instructions[*program_counter]);
                *program_counter += 1;
            }
            2 => {
                operand_values[index] = OperandValue::u16(((instructions[*program_counter] as u16) << 8) | instructions[*program_counter + 1] as u16);
                *program_counter += 2;
            }
            _ => {
                return Err(InstructionDecodeError::InvalidValueSize);
            }
        };
    }

    Ok(operand_values)
}

pub trait Instruction {
    // Also requires a `pub const OPCODE: u8`
    fn new(operand_values: OperandValues) -> Self where Self: Sized;
    fn name(&self) -> String;
    fn help(&self) -> String;
    fn signature(&self) -> String;
    fn identifier(&self) -> String;
    fn opcode(&self) -> u8;
    fn opcode_as_hex(&self) -> String {
        format!("{:#X}", self.opcode())
    }

    fn operand_map() -> OperandMap where Self: Sized;
    fn operand_values(&self) -> &OperandValues;
    fn set_operand_values(&mut self, operand_values: OperandValues);

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error>;
}