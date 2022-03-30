use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValue, OperandValues};
use crate::vm::{ExecutionResult, VM};

pub struct Load {
    operand_values: OperandValues,
}

impl Load {
    pub const OPCODE: u8 = 1;
}

impl Instruction for Load {
    fn new(operand_values: OperandValues) -> Self {
        Load { operand_values }
    }

    fn name(&self) -> String {
        "Load".to_string()
    }

    fn help(&self) -> String {
        "Loads constant value into a register".to_string()
    }

    fn signature(&self) -> String {
        "LOAD ${D} #{u16}".to_string()
    }

    fn identifier(&self) -> String {
        "LOAD".to_string()
    }

    fn opcode(&self) -> u8 {
        Load::OPCODE
    }

    fn operand_map() -> OperandMap {
        OperandMap::from([1, 2, 0])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let destination = match &self.operand_values[0] {
            OperandValue::u8(value) => *value as usize,
            OperandValue::u16(value) => *value as usize,
            OperandValue::None => panic!("Did not receive a destination register")
        };

        let number = match &self.operand_values[1] {
            OperandValue::u8(value) => *value as i32,
            OperandValue::u16(value) => *value as i32,
            OperandValue::None => panic!("Did not receive a value")
        };

        vm.set_register(destination, number);
        Ok(ExecutionResult::Value(number))
    }
}