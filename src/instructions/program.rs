use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
use crate::vm::{ExecutionResult, VM};

pub struct Jump {
    operand_values: OperandValues,
}

impl Jump {
    pub const OPCODE: u8 = 10;
}

impl Instruction for Jump {
    fn new(operand_values: OperandValues) -> Self {
        Jump { operand_values }
    }

    fn name(&self) -> String {
        "Jump".to_string()
    }

    fn help(&self) -> String {
        "Jumps to an absolute program position".to_string()
    }

    fn signature(&self) -> String {
        "JMP".to_string()
    }

    fn identifier(&self) -> String {
        "JMP".to_string()
    }

    fn opcode(&self) -> u8 {
        Jump::OPCODE
    }

    fn operand_map() -> OperandMap {
        OperandMap::from([2, 0, 0])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let destination = self.operand_values[0].as_program_index();

        vm.halt();
        Ok(ExecutionResult::Value(1))
    }
}