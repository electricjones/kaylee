use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
use crate::vm::{ExecutionResult, Kaylee};

pub struct Halt {
    operand_values: OperandValues,
}

impl Halt {
    pub const OPCODE: u8 = 0;
}

impl Instruction for Halt {
    fn new(operand_values: OperandValues) -> Self {
        Halt { operand_values }
    }

    fn name(&self) -> String {
        "Halt".to_string()
    }

    fn help(&self) -> String {
        "Kills the entire program".to_string()
    }

    fn signature(&self) -> String {
        "HALT".to_string()
    }

    fn identifier(&self) -> String {
        "HALT".to_string()
    }

    fn opcode(&self) -> u8 {
        Halt::OPCODE
    }

    fn operand_map() -> OperandMap {
        OperandMap::from([3, 0, 0])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        vm.halt();
        Ok(ExecutionResult::Halted)
    }
}