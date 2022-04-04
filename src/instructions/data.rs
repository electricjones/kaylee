use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
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
        let destination = self.operand_values[0].as_register_id();
        let value = self.operand_values[1].as_constant_value();

        vm.set_register(destination, value).unwrap();
        Ok(ExecutionResult::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::program::Program;
    use crate::vm::VM;

    #[test]
    fn test_load() {
        let mut program = Program::from([
            1, 4, 1, 244, // LOAD $4 #500
            1, 30, 0, 12,  // LOAD $6 #12
        ]);

        let mut vm = VM::new();
        vm.run(program);

        assert_eq!(500, vm.register(4).unwrap());
        assert_eq!(12, vm.register(30).unwrap());
    }
}