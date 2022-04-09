use std::fmt::Error;

use crate::instructions::compare::{Equal, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, NotEqual};
use crate::instructions::data::Load;
use crate::instructions::math::{Add, Divide, Multiply, Subtract};
use crate::instructions::program::{Jump, JumpBackward, JumpEqual, JumpForward};
use crate::instructions::system::Halt;
use crate::vm::{DoubleWord, ExecutionResult, FourWords, Program, ProgramIndex, RegisterId, RegisterValue, VM, Word};

mod system;
mod data;
mod math;
mod program;
mod compare;

type OperandMap = [usize; 3];

type OperandValues = [OperandValue; 3];

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OperandValue {
    u8(Word),
    u16(DoubleWord),
    u32(FourWords),
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
            OperandValue::u32(value) => *value as usize,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    fn as_program_index(&self) -> ProgramIndex {
        self.as_register_id() as ProgramIndex
    }

    fn as_constant_value(&self) -> RegisterValue {
        match self {
            OperandValue::u8(value) => *value as RegisterValue,
            OperandValue::u16(value) => *value as RegisterValue,
            OperandValue::u32(value) => *value as RegisterValue,
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }

    pub(crate) fn as_string(&self) -> String {
        match self {
            OperandValue::u8(value) => value.to_string(),
            OperandValue::u16(value) => value.to_string(),
            OperandValue::u32(value) => value.to_string(),
            OperandValue::None => panic!("Did not receive a destination register")
        }
    }
}

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

    let opcode: Word = instructions[*program_counter];
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


pub fn build<T: 'static + Instruction>(instructions: &Program, program_counter: &mut usize) -> Result<Box<dyn Instruction>, InstructionDecodeError> {
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

pub fn consume_and_parse_values(operand_map: OperandMap, instructions: &Program, program_counter: &mut usize) -> Result<OperandValues, InstructionDecodeError> {
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
            3 => {
                // @todo: This should really be u24
                let a = (instructions[*program_counter] as FourWords) << 16;
                let b = (instructions[*program_counter + 1] as FourWords) << 8;
                let c = instructions[*program_counter + 2] as FourWords;

                let value = (a | b | c) as u32;

                operand_values[index] = OperandValue::u32(value);

                *program_counter += 3;
            }
            _ => {
                return Err(InstructionDecodeError::InvalidValueSize);
            }
        };
    }

    Ok(operand_values)
}

fn basic_register_execution<I: Instruction, F: Fn(RegisterValue, RegisterValue) -> RegisterValue>(instruction: &I, vm: &mut VM, callback: F) -> RegisterValue {
    let destination = instruction.operand_values()[0].as_register_id();

    let left = instruction.get_register_value_for_operand(1, vm).unwrap();
    let right = instruction.get_register_value_for_operand(2, vm).unwrap();

    let result = callback(left, right);

    vm.set_register(destination, result as RegisterValue).unwrap();
    result
}

pub trait Instruction {
    // Also requires a `pub const OPCODE: u8`
    fn new(operand_values: OperandValues) -> Self where Self: Sized;
    // fn raw(&self) -> [u8; 4];
    fn name(&self) -> String;
    fn help(&self) -> String;

    // @todo: I think I can bring `signature(), operand_map()` together
    // @todo: `signature() -> ["DIV", ("$", "D"), ("#", "NUMBER")]`
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

    fn display(&self) -> String {
        let signature = self.signature();
        let pieces = signature.split(" ");

        let mut output = String::new();
        for (key, piece) in pieces.into_iter().enumerate() {
            if key == 0 {
                output.push_str(piece);
                continue;
            }

            output.push(' ');
            output.push(piece.chars().nth(0).unwrap());

            let value = &self.operand_values()[key - 1].as_string();

            output.push_str(value.as_str());
        }

        output
    }

    fn get_register_value_for_operand(&self, operand_value_index: usize, vm: &mut VM) -> Result<RegisterValue, ()> {
        let register = self.operand_values()[operand_value_index].as_register_id();
        vm.register(register)
    }
}


#[cfg(test)]
mod tests {
    use crate::instructions::{consume_and_parse_values, OperandMap, OperandValue};

    #[test]
    fn consume_and_parse_values_0() {
        let operand_map = OperandMap::from([0, 0, 0]);
        let mut instructions: Vec<u8> = vec![0, 0, 0];

        let expected = [
            OperandValue::None,
            OperandValue::None,
            OperandValue::None,
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 0);
    }

    #[test]
    fn consume_and_parse_values_all_1() {
        let operand_map = OperandMap::from([1, 1, 1]);
        let mut instructions: Vec<u8> = vec![30, 40, 50];

        let expected = [
            OperandValue::u8(30),
            OperandValue::u8(40),
            OperandValue::u8(50),
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
    }

    #[test]
    fn consume_and_parse_values_2_1() {
        let operand_map = OperandMap::from([2, 1, 0]);
        let mut instructions: Vec<u8> = vec![1, 244, 100];

        let expected = [
            OperandValue::u16(500),
            OperandValue::u8(100),
            OperandValue::None
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
    }

    #[test]
    fn consume_and_parse_values_1_0_0() {
        let operand_map = OperandMap::from([1, 0, 0]);
        let mut instructions: Vec<u8> = vec![70];

        let expected = [
            OperandValue::u8(70),
            OperandValue::None,
            OperandValue::None
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 1);
    }

    #[test]
    fn consume_and_parse_values_1_1_0() {
        let operand_map = OperandMap::from([1, 1, 0]);
        let mut instructions: Vec<u8> = vec![70, 80];

        let expected = [
            OperandValue::u8(70),
            OperandValue::u8(80),
            OperandValue::None
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 2);
    }

    #[test]
    fn consume_and_parse_values_a_single_24_bit() {
        let operand_map = OperandMap::from([3, 0, 0]);
        let mut instructions: Vec<u8> = vec![1, 1, 1];

        let expected = [
            OperandValue::u32(65793), // 00 01 01 01
            OperandValue::None,
            OperandValue::None
        ];

        test_consume_and_parse_values(operand_map, &mut instructions, &expected, 3);
    }

    fn test_consume_and_parse_values(operand_map: [usize; 3], mut instructions: &mut Vec<u8>, expected: &[OperandValue; 3], expected_counter: usize) {
        let mut counter: usize = 0;

        let result = consume_and_parse_values(operand_map, &mut instructions, &mut counter).unwrap();
        for (key, value) in result.iter().enumerate() {
            assert_eq!(&expected[key], value, "failed on value {}", key);
        }

        assert_eq!(expected_counter, counter);
    }
}