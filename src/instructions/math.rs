use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
use crate::vm::{ExecutionResult, VM};

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

        vm.set_register(destination, value).unwrap();
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

        vm.set_register(destination, value).unwrap();
        Ok(ExecutionResult::Value(value))
    }
}


pub struct Multiply {
    operand_values: OperandValues,
}

impl Multiply {
    pub const OPCODE: u8 = 4;
}

impl Instruction for Multiply {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        Multiply {
            operand_values
        }
    }

    fn name(&self) -> String {
        "Multiply".to_string()
    }

    fn help(&self) -> String {
        "Multiplies to numbers together".to_string()
    }

    fn signature(&self) -> String {
        "MUL $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "MUL".to_string()
    }

    fn opcode(&self) -> u8 {
        Multiply::OPCODE
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

        let value = left * right;

        vm.set_register(destination, value).unwrap();
        Ok(ExecutionResult::Value(value))
    }
}


pub struct Divide {
    operand_values: OperandValues,
}

impl Divide {
    pub const OPCODE: u8 = 5;
}

impl Instruction for Divide {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        Divide {
            operand_values
        }
    }

    fn name(&self) -> String {
        "Divide".to_string()
    }

    fn help(&self) -> String {
        "Divides to numbers".to_string()
    }

    fn signature(&self) -> String {
        "DIV $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "DIV".to_string()
    }

    fn opcode(&self) -> u8 {
        Divide::OPCODE
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

        let value = left / right;
        let remainder = (left % right) as u32;

        vm.set_register(destination, value).unwrap();
        vm.set_remainder(remainder);

        Ok(ExecutionResult::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::math::{Add, Divide, Multiply, Subtract};
    use crate::vm::Program;
    use crate::vm::VM;

    #[test]
    fn test_add() {
        let program = Program::from([
            2, 29, 0, 2,
            2, 30, 1, 3,
            2, 31, 29, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 12).unwrap();
        vm.set_register(1, 10).unwrap();
        vm.set_register(2, 500).unwrap();
        vm.set_register(3, 7).unwrap();

        // $29[512] = 12 + 500
        // $30[17] = 10 + 7
        // $31[529] = 512 + 17

        vm.run(program);

        assert_eq!(512, vm.register(29).unwrap());
        assert_eq!(17, vm.register(30).unwrap());
        assert_eq!(529, vm.register(31).unwrap());
    }

    #[test]
    fn test_subtract() {
        let program = Program::from([
            Subtract::OPCODE, 29, 0, 2,
            Subtract::OPCODE, 30, 1, 3,
            Subtract::OPCODE, 31, 29, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 222).unwrap();
        vm.set_register(1, 14).unwrap();
        vm.set_register(2, 22).unwrap();
        vm.set_register(3, 3).unwrap();

        // $29[200] = 222 - 22
        // $30[11] = 14 - 3
        // $31[189] = 200 - 11

        vm.run(program);

        assert_eq!(200, vm.register(29).unwrap());
        assert_eq!(11, vm.register(30).unwrap());
        assert_eq!(189, vm.register(31).unwrap());
    }

    #[test]
    fn test_multiply() {
        let program = Program::from([
            Multiply::OPCODE, 29, 0, 2,
            Multiply::OPCODE, 30, 1, 3,
            Multiply::OPCODE, 31, 29, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 2).unwrap();
        vm.set_register(1, 4).unwrap();
        vm.set_register(2, 6).unwrap();
        vm.set_register(3, 8).unwrap();

        // $29[12] = 2 * 6
        // $30[32] = 4 * 8
        // $31[384] = 12 * 32

        vm.run(program);

        assert_eq!(12, vm.register(29).unwrap());
        assert_eq!(32, vm.register(30).unwrap());
        assert_eq!(384, vm.register(31).unwrap());
    }

    #[test]
    fn test_divide_no_remainder() {
        let program = Program::from([
            Divide::OPCODE, 31, 0, 1,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 16).unwrap();
        vm.set_register(1, 2).unwrap();

        vm.run(program);

        assert_eq!(8, vm.register(31).unwrap());
        assert_eq!(0, vm.remainder());
    }

    #[test]
    fn test_divide_with_remainder() {
        let program = Program::from([
            Divide::OPCODE, 31, 0, 1,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 13).unwrap();
        vm.set_register(1, 5).unwrap();

        vm.run(program);

        assert_eq!(2, vm.register(31).unwrap());
        assert_eq!(3, vm.remainder());
    }

    #[test]
    fn test_math() {
        let program = Program::from([
            Add::OPCODE, 29, 0, 2,
            Add::OPCODE, 30, 29, 2,
            Subtract::OPCODE, 30, 29, 1,
            Add::OPCODE, 28, 3, 4,
            Multiply::OPCODE, 31, 3, 2,
            Divide::OPCODE, 3, 29, 30,
            Subtract::OPCODE, 4, 2, 30,
            Add::OPCODE, 0, 3, 28,
            Multiply::OPCODE, 1, 3, 4,
            Divide::OPCODE, 31, 28, 30,
        ]);

        let mut vm = VM::new();
        vm.set_register(0, 2).unwrap();
        vm.set_register(1, 4).unwrap();
        vm.set_register(2, 6).unwrap();
        vm.set_register(3, 8).unwrap();
        vm.set_register(4, 9).unwrap();

        // 29[8] = 2 + 6
        // 30[14] = 8 + 6
        // 30[4] = 8 - 4
        // 28[17] = 8 + 9
        // 31[48] = 8 * 6
        // 3[2] = 8 / 4
        // 4[2] = 6 - 4
        // 0[19] = 2 + 17
        // 1[4] = 2 * 2
        // 31[4r1] = 17 / 4

        vm.run(program);

        assert_eq!(19, vm.register(0).unwrap());
        assert_eq!(4, vm.register(1).unwrap());
        assert_eq!(6, vm.register(2).unwrap());
        assert_eq!(2, vm.register(3).unwrap());
        assert_eq!(2, vm.register(4).unwrap());
        assert_eq!(17, vm.register(28).unwrap());
        assert_eq!(8, vm.register(29).unwrap());
        assert_eq!(4, vm.register(30).unwrap());
        assert_eq!(4, vm.register(31).unwrap());
        assert_eq!(1, vm.remainder());
    }
}
