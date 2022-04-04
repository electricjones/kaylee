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


pub struct Subtract {
    operand_values: OperandValues,
}

impl Subtract {
    pub const OPCODE: u8 = 3;
}

impl Instruction for Subtract {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        Subtract {
            operand_values
        }
    }

    fn name(&self) -> String {
        "Subtract".to_string()
    }

    fn help(&self) -> String {
        "Subtracts to numbers from each other".to_string()
    }

    fn signature(&self) -> String {
        "SUB $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "SUB".to_string()
    }

    fn opcode(&self) -> u8 {
        Subtract::OPCODE
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

        let value = left - right;

        vm.set_register(destination, value);
        Ok(ExecutionResult::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::math::Subtract;
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

    #[test]
    fn test_subtract() {
        let mut program = Program::new(vec![
            Subtract::OPCODE, 29, 0, 2,
            Subtract::OPCODE, 30, 1, 3,
            Subtract::OPCODE, 31, 29, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 222);
        vm.set_register(1, 14);
        vm.set_register(2, 22);
        vm.set_register(3, 3);

        // $29[200] = 222 - 22
        // $30[11] = 14 - 3
        // $31[189] = 200 - 11

        vm.run(&mut program);

        assert_eq!(200, vm.register(29).unwrap());
        assert_eq!(11, vm.register(30).unwrap());
        assert_eq!(189, vm.register(31).unwrap());
    }
}
