use std::fmt::Error;

use crate::vm::{ExecutionResult, VM};

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

    let instruction = match opcode {
        Halt::OPCODE => {
            Halt::new(
                consume_and_parse_values(
                    Halt::operand_map(),
                    instructions,
                    program_counter,
                )?
            )
        }
        _ => {
            return Err(InstructionDecodeError::IllegalOpcode);
        }
    };

    Ok(Box::new(instruction))
}


pub fn consume_and_parse_values(operand_map: OperandMap, instructions: &mut Vec<u8>, program_counter: &mut usize) -> Result<OperandValues, InstructionDecodeError> {
    let mut operand_values: OperandValues = [OperandValue::None, OperandValue::None, OperandValue::None];

    for (index, bytes) in operand_map.iter().enumerate() {
        match bytes {
            0 => {
                operand_values[index] = OperandValue::None;
            }
            1 => {
                operand_values[index] = OperandValue::u8(instructions[*program_counter + 1]);
                *program_counter += 1;
            }
            2 => {
                operand_values[index] = OperandValue::u16(((instructions[*program_counter + 1] as u16) << 8) | instructions[*program_counter + 2] as u16);
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
    fn name(&self) -> String;
    fn description(&self) -> String;
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

pub struct Halt {
    operand_values: OperandValues,
}

impl Halt {
    pub const OPCODE: u8 = 0;
    pub fn new(operand_values: OperandValues) -> Self {
        Halt { operand_values }
    }
}

impl Instruction for Halt {
    fn name(&self) -> String {
        "Halt".to_string()
    }

    fn description(&self) -> String {
        "Kills the entire program".to_string()
    }

    fn identifier(&self) -> String {
        "HALT".to_string()
    }

    fn opcode(&self) -> u8 {
        Halt::OPCODE
    }

    fn operand_map() -> OperandMap {
        OperandMap::from([0, 0, 0])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        vm.halt();
        Ok(ExecutionResult::Halted)
    }
}
//
// pub struct Load {}
//
// impl Instruction for Load {
//     fn name(&self) -> &str {
//         "Load"
//     }
//
//     fn description(&self) -> &str {
//         "Load a value into a register"
//     }
//
//     fn identifier(&self) -> &str {
//         "LOAD"
//     }
//
//     fn static_opcode() -> i32 {
//         1
//     }
//
//     fn operands(&self) -> (usize, usize, usize) {
//         (8, 16, 0)
//     }
//
//     fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
//         // @todo: I don't like getting the bits like this
//         let register = vm.next_8_bits() as usize;
//         let number = vm.next_16_bits() as u16;
//
//         // @todo: turn this into a setter
//         // vm.set_register(register, number as i32);
//         // vm.jump(index);
//         // vm.halt(exit_code);
//         vm.registers[register] = number as i32;
//
//         Ok(ExecutionResult::Value(number as i32))
//     }
// }