use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValue, OperandValues};
use crate::vm::{ExecutionResult, RegisterValue, VM};

pub struct Add {
    operand_values: OperandValues,
}

impl Add {
    pub const OPCODE: u8 = 2;
}

impl Instruction for Add {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        Add {
            operand_values
        }
    }

    fn name(&self) -> String {
        "Add".to_string()
    }

    fn help(&self) -> String {
        "Adds to numbers together".to_string()
    }

    fn signature(&self) -> String {
        "ADD $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "ADD".to_string()
    }

    fn opcode(&self) -> u8 {
        Add::OPCODE
    }

    fn operand_map() -> OperandMap where Self: Sized {
        OperandMap::from([1, 1, 1])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let destination = self.operand_values[0].as_register_id();

        let left = self.get_register_value_for_operand(1, vm).unwrap();
        let right = self.get_register_value_for_operand(2, vm).unwrap();

        let value = left + right;

        vm.set_register(destination, value);
        Ok(ExecutionResult::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::program::Program;
    use crate::vm::VM;

    #[test]
    fn test_add() {
        let mut program = Program::new(vec![
            2, 29, 0, 2,
            2, 30, 1, 3,
            2, 31, 29, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 12);
        vm.set_register(1, 10);
        vm.set_register(2, 500);
        vm.set_register(3, 7);

        // $29[512] = 12 + 500
        // $30[17] = 10 + 7
        // $31[529] = 512 + 17

        vm.run(&mut program);

        assert_eq!(512, vm.register(29).unwrap());
        assert_eq!(17, vm.register(30).unwrap());
        assert_eq!(529, vm.register(31).unwrap());
    }
}