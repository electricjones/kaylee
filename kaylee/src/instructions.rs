use std::fmt::Error;

use linkme::distributed_slice;

use crate::instructions::compare::{Equal, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, NotEqual};
use crate::instructions::data::Load;
use crate::instructions::machine::Halt;
use crate::instructions::math::{Add, Divide, Multiply, Subtract};
use crate::instructions::program::{Jump, JumpBackward, JumpEqual, JumpForward};
use crate::program::{Program, ProgramIndex};
use crate::vm::{Byte, ExecutionResult, HalfWord, Kaylee, RegisterId, RegisterValue, Word};

mod machine;
mod data;
mod math;
mod program;
mod compare;
mod logical;
mod system;
mod library;
mod misc;

/// Type for the three operand slots allowed for each instruction
pub type RegisteredInstruction = (&'static str, u8, [OperandType; 3]);

/// Data Repository for Registered Instructions. 
/// Not intended to be directly accessed. Use `InstructionRegistry` instead.
#[distributed_slice]
pub static _INSTRUCTION_REGISTRY: [RegisteredInstruction] = [..];

/// Link-time built registry of all allowed instructions, including signatures.
/// Useful for parsing, listing, and examining instructions
pub struct InstructionRegistry {}

impl InstructionRegistry {
    /// Get a RegisteredInstruction from the InstructionRegistry if it exists
    pub fn get(operation: &str) -> Option<&RegisteredInstruction> {
        let mut item: Option<&RegisteredInstruction> = None;
        for registered_instruction in _INSTRUCTION_REGISTRY {
            if registered_instruction.0 == operation {
                item = Some(registered_instruction);
                break;
            }
        }

        item
    }
}

/// Errors concerning decoding instruction bytecode
#[derive(Debug)]
pub enum InstructionDecodeError {
    InvalidValueSize,
    IllegalOpcode,
}

/// Decode the next instruction in the Program stream
pub fn decode_next_instruction(instructions: &Program, program_counter: &mut usize) -> Option<Result<Box<dyn Instruction>, InstructionDecodeError>> {
    // @todo: I am not super happy with this decoding scheme. It should probably grab the entire slice (4 bytes) and handle them together
    if *program_counter >= instructions.len() {
        return None;
    }

    let opcode: Byte = instructions[*program_counter];
    *program_counter += 1;

    Some(match opcode {
        Halt::OPCODE => build::<Halt>(instructions, program_counter),
        Load::OPCODE => build::<Load>(instructions, program_counter),

        Add::OPCODE => build::<Add>(instructions, program_counter),
        Subtract::OPCODE => build::<Subtract>(instructions, program_counter),
        Multiply::OPCODE => build::<Multiply>(instructions, program_counter),
        Divide::OPCODE => build::<Divide>(instructions, program_counter),

        Jump::OPCODE => build::<Jump>(instructions, program_counter),
        JumpForward::OPCODE => build::<JumpForward>(instructions, program_counter),
        JumpBackward::OPCODE => build::<JumpBackward>(instructions, program_counter),
        JumpEqual::OPCODE => build::<JumpEqual>(instructions, program_counter),

        Equal::OPCODE => build::<Equal>(instructions, program_counter),
        NotEqual::OPCODE => build::<NotEqual>(instructions, program_counter),
        GreaterThan::OPCODE => build::<GreaterThan>(instructions, program_counter),
        LessThan::OPCODE => build::<LessThan>(instructions, program_counter),
        GreaterThanOrEqual::OPCODE => build::<GreaterThanOrEqual>(instructions, program_counter),
        LessThanOrEqual::OPCODE => build::<LessThanOrEqual>(instructions, program_counter),

        _ => {
            Err(InstructionDecodeError::IllegalOpcode)
        }
    })
}

/// Build the Instruction TraitObject from the program stream
pub fn build<T: 'static + Instruction>(instructions: &Program, program_counter: &mut usize) -> Result<Box<dyn Instruction>, InstructionDecodeError> {
    Ok(
        Box::new(
            T::new(
                consume_and_parse_values(
                    T::signature(),
                    instructions,
                    program_counter,
                )?
            )
        )
    )
}

/// Decodes the operand values from the Instruction Stream
pub fn consume_and_parse_values(signature: InstructionSignature, instructions: &Program, program_counter: &mut usize) -> Result<OperandValues, InstructionDecodeError> {
    let mut operand_values: OperandValues = [OperandValue::None, OperandValue::None, OperandValue::None];

    let original_pc = *program_counter;
    for (index, bytes) in signature.operands.iter().enumerate() {
        match bytes {
            OperandType::None => {
                operand_values[index] = OperandValue::None;
            }
            OperandType::RegisterId | OperandType::ConstantByte => {
                operand_values[index] = OperandValue::Byte(instructions[*program_counter]);
                *program_counter += 1;
            }
            OperandType::ConstantHalfWord => {
                operand_values[index] = OperandValue::HalfWord(((instructions[*program_counter] as HalfWord) << 8) | instructions[*program_counter + 1] as u16);
                *program_counter += 2;
            }
            OperandType::ConstantWord => {
                // @todo: This should really be u24
                let a = (instructions[*program_counter] as Word) << 16;
                let b = (instructions[*program_counter + 1] as Word) << 8;
                let c = instructions[*program_counter + 2] as Word;

                let value = (a | b | c) as u32;

                operand_values[index] = OperandValue::Word(value);

                *program_counter += 3;
            }
        };
    }

    if (original_pc + 3) != *program_counter {
        *program_counter = original_pc + 3;
    }
    Ok(operand_values)
}

/// Prints an instruction in an Instruction Stream in a human readable format
pub fn display_instruction_with_values<T: 'static + Instruction>(instruction: &T) -> String {
    let mut output = String::new();
    output.push_str(T::signature().identifier.as_str());

    for (index, operand_type) in T::signature().operands.iter().enumerate() {
        match operand_type {
            OperandType::None => {}
            OperandType::RegisterId => {
                let value = instruction.operand_value(index).unwrap().as_constant_value();
                output.push_str(format!(" ${value}").as_str())
            }
            _ => {
                let value = instruction.operand_value(index).unwrap().as_constant_value();
                output.push_str(format!(" #{value}").as_str())
            }
        }
    }

    output
}

/// Helper for a common instruction execution. Executes callback with the values from two operands, setting a destination register
fn basic_register_execution<I: Instruction, F: Fn(RegisterValue, RegisterValue) -> RegisterValue>(instruction: &I, vm: &mut Kaylee, callback: F) -> RegisterValue {
    let destination = instruction.operand_values()[0].as_register_id();

    let left = instruction.get_register_value_for_operand(1, vm).unwrap();
    let right = instruction.get_register_value_for_operand(2, vm).unwrap();

    let result = callback(left, right);

    vm.set_register(destination, result as RegisterValue).unwrap();
    result
}

/// Potential types of Operands
pub enum OperandType {
    None,
    RegisterId,
    ConstantByte,
    ConstantHalfWord,
    ConstantWord,
}

type OperandValues = [OperandValue; 3];

/// Value for an operand in the Instruction Stream
#[derive(PartialOrd, PartialEq, Debug)]
pub enum OperandValue {
    Byte(Byte),
    HalfWord(HalfWord),
    Word(Word),
    None,
}

/// Decode a single operand from a byte in the Instruction Stream
impl TryFrom<Byte> for OperandValue {
    type Error = ();

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        Ok(OperandValue::Byte(value))
    }
}

/// Decode a single operand from a Halfword in the Instruction Stream
impl TryFrom<HalfWord> for OperandValue {
    type Error = ();

    fn try_from(value: HalfWord) -> Result<Self, Self::Error> {
        Ok(OperandValue::HalfWord(value))
    }
}

impl OperandValue {
    /// Get the OperandValue as a RegisterId
    // @todo: I tried to do these conversions using TryFrom and a generic `into<T>(&self) -> T` function, but neither worked.
    // @todo: There is certainly a more idiomatic way
    fn as_register_id(&self) -> RegisterId {
        match self {
            OperandValue::Byte(value) => *value as usize,
            OperandValue::HalfWord(value) => *value as usize,
            OperandValue::Word(value) => *value as usize,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    /// Get the OperandValue as a Program Index Target
    fn as_program_index(&self) -> ProgramIndex {
        self.as_register_id() as ProgramIndex
    }

    /// Get the OperandValue as a constant literal value (integer)
    fn as_constant_value(&self) -> RegisterValue {
        match self {
            OperandValue::Byte(value) => *value as RegisterValue,
            OperandValue::HalfWord(value) => *value as RegisterValue,
            OperandValue::Word(value) => *value as RegisterValue,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    /// Get the OperandValue as a string
    pub(crate) fn as_string(&self) -> String {
        match self {
            OperandValue::Byte(value) => value.to_string(),
            OperandValue::HalfWord(value) => value.to_string(),
            OperandValue::Word(value) => value.to_string(),
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }
}

/// Defines an Instruction's Signature
pub struct InstructionSignature {
    pub identifier: String,
    pub operands: [OperandType; 3],
}

/// Defines an Instruction's documentation
pub struct InstructionDocumentation {
    pub name: String,
    pub help: String,
}

/// Allows an Instruction to be executable
pub trait Executable {
    // @todo: The only thing (other than the OPCODE constant) that is actually required w/o macro
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error>;
}

/// Defines the Instruction itself
/// This is built automatically with the derive(Instruction) macro
pub trait Instruction: Executable {
    // Also requires a `pub const OPCODE: u8`

    /// Create a new instruction with Concrete Values
    fn new(operand_values: OperandValues) -> Self where Self: Sized;

    /// Return the Instruction Signature
    fn signature() -> InstructionSignature where Self: Sized;

    /// Return the Instruction Documentation
    fn documentation() -> InstructionDocumentation where Self: Sized;

    /// Return a human-readable form of the instruction
    fn display(&self) -> String;

    /// Return the concrete OperandValues
    fn operand_values(&self) -> &OperandValues;

    /// Return a specific, concrete OperandValue
    fn operand_value(&self, index: usize) -> Result<&OperandValue, String> {
        if index > 2 {
            return Err("Index Out Of Bounds".to_string());
        }

        Ok(&self.operand_values()[index])
    }

    /// Get a concrete value from a register by looking at the target in an OperandValue
    fn get_register_value_for_operand(&self, operand_value_index: usize, vm: &mut Kaylee) -> Result<RegisterValue, ()> {
        let register = self.operand_values()[operand_value_index].as_register_id();
        vm.register(register)
    }
}