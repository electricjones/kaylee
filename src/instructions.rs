use std::fmt::Error;

use crate::instructions::data::Load;
use crate::instructions::math::{Add, Subtract};
use crate::instructions::system::Halt;
use crate::vm::{DoubleWord, ExecutionResult, RegisterId, RegisterValue, VM, Word};

mod system;
mod data;
mod math;

type OperandMap = [usize; 3];

type OperandValues = [OperandValue; 3];

pub enum OperandValue {
    u8(Word),
    u16(DoubleWord),
    None,
}

impl TryFrom<Word> for OperandValue {
    type Error = ();

    fn try_from(value: Word) -> Result<Self, Self::Error> {
        Ok(OperandValue::u8(value))
    }
}

impl TryFrom<DoubleWord> for OperandValue {
    type Error = ();

    fn try_from(value: DoubleWord) -> Result<Self, Self::Error> {
        Ok(OperandValue::u16(value))
    }
}

impl OperandValue {
    // @todo: I tried to do these conversions using TryFrom and a generic `into<T>(&self) -> T` function, but neither worked.
    // @todo: There is certainly a more idiomatic way
    fn as_register_id(&self) -> RegisterId {
        match self {
            OperandValue::u8(value) => *value as usize,
            OperandValue::u16(value) => *value as usize,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    fn as_constant_value(&self) -> RegisterValue {
        match self {
            OperandValue::u8(value) => *value as RegisterValue,
            OperandValue::u16(value) => *value as RegisterValue,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }
}

#[derive(Debug)]
pub enum InstructionDecodeError {
    InvalidValueSize,
    IllegalOpcode,
}

pub fn decode_next_instruction(instructions: &mut Vec<Word>, program_counter: &mut usize) -> Result<Box<dyn Instruction>, InstructionDecodeError> {
    let opcode: Word = instructions[*program_counter];
    *program_counter += 1;

    match opcode {
        Halt::OPCODE => build::<Halt>(instructions, program_counter),
        Load::OPCODE => build::<Load>(instructions, program_counter),
        Add::OPCODE => build::<Add>(instructions, program_counter),
        Subtract::OPCODE => build::<Subtract>(instructions, program_counter),
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
                operand_values[index] = OperandValue::u16(((instructions[*program_counter] as DoubleWord) << 8) | instructions[*program_counter + 1] as u16);
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
    fn opcode(&self) -> Word;
    fn opcode_as_hex(&self) -> String {
        format!("{:#X}", self.opcode())
    }

    fn operand_map() -> OperandMap where Self: Sized;
    fn operand_values(&self) -> &OperandValues;
    fn set_operand_values(&mut self, operand_values: OperandValues);

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error>;

    fn get_register_value_for_operand(&self, operand_value_index: usize, vm: &mut VM) -> Result<RegisterValue, ()> {
        let register = self.operand_values()[operand_value_index].as_register_id();
        vm.register(register)
    }
}