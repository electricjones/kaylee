use std::fmt::Error;

// use crate::instructions::compare::{Equal, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, NotEqual};
use crate::instructions::data::Load;
// use crate::instructions::math::{Add, Divide, Multiply, Subtract};
// use crate::instructions::program::{Jump, JumpBackward, JumpEqual, JumpForward};
// use crate::instructions::system::Halt;
use crate::vm::{Byte, ExecutionResult, HalfWord, Kaylee, Program, ProgramIndex, RegisterId, RegisterValue, Word};

mod system;
mod data;
mod math;
mod program;
mod compare;

#[derive(Debug)]
pub enum InstructionDecodeError {
    InvalidValueSize,
    IllegalOpcode,
}

// @todo: does this return type need to be so complex? Probably
pub fn decode_next_instruction(instructions: &Program, program_counter: &mut usize) -> Option<Result<Box<dyn Instruction>, InstructionDecodeError>> {
    if *program_counter >= instructions.len() {
        return None;
    }

    let opcode: Byte = instructions[*program_counter];
    *program_counter += 1;

    Some(match opcode {
        // Halt::OPCODE => build::<Halt>(instructions, program_counter),
        Load::OPCODE => build::<Load>(instructions, program_counter),

        // Add::OPCODE => build::<Add>(instructions, program_counter),
        // Subtract::OPCODE => build::<Subtract>(instructions, program_counter),
        // Multiply::OPCODE => build::<Multiply>(instructions, program_counter),
        // Divide::OPCODE => build::<Divide>(instructions, program_counter),
        //
        // Jump::OPCODE => build::<Jump>(instructions, program_counter),
        // JumpForward::OPCODE => build::<JumpForward>(instructions, program_counter),
        // JumpBackward::OPCODE => build::<JumpBackward>(instructions, program_counter),
        // JumpEqual::OPCODE => build::<JumpEqual>(instructions, program_counter),
        //
        // Equal::OPCODE => build::<Equal>(instructions, program_counter),
        // NotEqual::OPCODE => build::<NotEqual>(instructions, program_counter),
        // GreaterThan::OPCODE => build::<GreaterThan>(instructions, program_counter),
        // LessThan::OPCODE => build::<LessThan>(instructions, program_counter),
        // GreaterThanOrEqual::OPCODE => build::<GreaterThanOrEqual>(instructions, program_counter),
        // LessThanOrEqual::OPCODE => build::<LessThanOrEqual>(instructions, program_counter),

        _ => {
            Err(InstructionDecodeError::IllegalOpcode)
        }
    })
}


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

pub fn consume_and_parse_values(signature: InstructionSignature, instructions: &Program, program_counter: &mut usize) -> Result<OperandValues, InstructionDecodeError> {
    let mut operand_values: OperandValues = [OperandValue::None, OperandValue::None, OperandValue::None];

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
            _ => {
                return Err(InstructionDecodeError::InvalidValueSize);
            }
        };
    }

    Ok(operand_values)
}

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

fn basic_register_execution<I: Instruction, F: Fn(RegisterValue, RegisterValue) -> RegisterValue>(instruction: &I, vm: &mut Kaylee, callback: F) -> RegisterValue {
    let destination = instruction.operand_values()[0].as_register_id();

    let left = instruction.get_register_value_for_operand(1, vm).unwrap();
    let right = instruction.get_register_value_for_operand(2, vm).unwrap();

    let result = callback(left, right);

    vm.set_register(destination, result as RegisterValue).unwrap();
    result
}

pub enum OperandType {
    None,
    RegisterId,
    ConstantByte,
    ConstantHalfWord,
    ConstantWord,
}

type OperandValues = [OperandValue; 3];

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OperandValue {
    Byte(Byte),
    HalfWord(HalfWord),
    Word(Word),
    None,
}

impl TryFrom<Byte> for OperandValue {
    type Error = ();

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        Ok(OperandValue::Byte(value))
    }
}

impl TryFrom<HalfWord> for OperandValue {
    type Error = ();

    fn try_from(value: HalfWord) -> Result<Self, Self::Error> {
        Ok(OperandValue::HalfWord(value))
    }
}

impl OperandValue {
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

    fn as_program_index(&self) -> ProgramIndex {
        self.as_register_id() as ProgramIndex
    }

    fn as_constant_value(&self) -> RegisterValue {
        match self {
            OperandValue::Byte(value) => *value as RegisterValue,
            OperandValue::HalfWord(value) => *value as RegisterValue,
            OperandValue::Word(value) => *value as RegisterValue,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    pub(crate) fn as_string(&self) -> String {
        match self {
            OperandValue::Byte(value) => value.to_string(),
            OperandValue::HalfWord(value) => value.to_string(),
            OperandValue::Word(value) => value.to_string(),
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }
}

pub struct InstructionSignature {
    pub identifier: String,
    pub operands: [OperandType; 3],
}

pub struct InstructionDocumentation {
    pub name: String,
    pub help: String,
}

pub trait Executable {
    // @todo: The only thing (other than the OPCODE constant) that is actually required w/o macro
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error>;
}

pub trait Instruction: Executable {
    // Also requires a `pub const OPCODE: u8`

    // @todo: These can be derived with a macro, eventually
    fn new(operand_values: OperandValues) -> Self where Self: Sized;
    fn signature() -> InstructionSignature where Self: Sized;
    fn documentation() -> InstructionDocumentation where Self: Sized;

    // @Todo: macro eligible
    fn display(&self) -> String;

    // fn raw(&self) -> [u8; 4];
    // fn opcode_as_hex(&self) -> String {
    //     format!("{:#X}", self.opcode())
    // }
    // @todo: macro eligible
    fn operand_values(&self) -> &OperandValues;


    fn operand_value(&self, index: usize) -> Result<&OperandValue, String> {
        if index > 2 {
            return Err("Index Out Of Bounds".to_string());
        }

        Ok(&self.operand_values()[index])
    }

    fn get_register_value_for_operand(&self, operand_value_index: usize, vm: &mut Kaylee) -> Result<RegisterValue, ()> {
        let register = self.operand_values()[operand_value_index].as_register_id();
        vm.register(register)
    }
}
//
// #[cfg(test)]
// mod tests {
//     use crate::instructions::{consume_and_parse_values, OperandMap, OperandValue};
//
//     #[test]
//     fn consume_and_parse_values_0() {
//         let operand_map = OperandMap::from([0, 0, 0]);
//         let mut instructions: Vec<u8> = vec![0, 0, 0];
//
//         let expected = [
//             OperandValue::None,
//             OperandValue::None,
//             OperandValue::None,
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 0);
//     }
//
//     #[test]
//     fn consume_and_parse_values_all_1() {
//         let operand_map = OperandMap::from([1, 1, 1]);
//         let mut instructions: Vec<u8> = vec![30, 40, 50];
//
//         let expected = [
//             OperandValue::Byte(30),
//             OperandValue::Byte(40),
//             OperandValue::Byte(50),
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
//     }
//
//     #[test]
//     fn consume_and_parse_values_2_1() {
//         let operand_map = OperandMap::from([2, 1, 0]);
//         let mut instructions: Vec<u8> = vec![1, 244, 100];
//
//         let expected = [
//             OperandValue::HalfWord(500),
//             OperandValue::Byte(100),
//             OperandValue::None
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
//     }
//
//     #[test]
//     fn consume_and_parse_values_1_0_0() {
//         let operand_map = OperandMap::from([1, 0, 0]);
//         let mut instructions: Vec<u8> = vec![70];
//
//         let expected = [
//             OperandValue::Byte(70),
//             OperandValue::None,
//             OperandValue::None
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 1);
//     }
//
//     #[test]
//     fn consume_and_parse_values_1_1_0() {
//         let operand_map = OperandMap::from([1, 1, 0]);
//         let mut instructions: Vec<u8> = vec![70, 80];
//
//         let expected = [
//             OperandValue::Byte(70),
//             OperandValue::Byte(80),
//             OperandValue::None
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 2);
//     }
//
//     #[test]
//     fn consume_and_parse_values_a_single_24_bit() {
//         let operand_map = OperandMap::from([3, 0, 0]);
//         let mut instructions: Vec<u8> = vec![1, 1, 1];
//
//         let expected = [
//             OperandValue::Word(65793), // 00 01 01 01
//             OperandValue::None,
//             OperandValue::None
//         ];
//
//         test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
//     }
//
//     fn test_consume_and_parse_values(operand_map: [usize; 3], mut instructions: &mut Vec<u8>, expected: &[OperandValue; 3], expected_counter: usize) {
//         let mut counter: usize = 0;
//
//         let result = consume_and_parse_values(operand_map, &mut instructions, &mut counter).unwrap();
//         for (key, value) in result.iter().enumerate() {
//             assert_eq!(&expected[key], value, "failed on value {}", key);
//         }
//
//         assert_eq!(expected_counter, counter);
//     }
// }