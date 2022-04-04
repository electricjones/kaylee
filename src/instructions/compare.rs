use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
use crate::vm::{ExecutionResult, RegisterValue, VM};

pub struct Equal {
    operand_values: OperandValues,
}

impl Equal {
    pub const OPCODE: u8 = 20;
}

impl Instruction for Equal {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        Equal {
            operand_values
        }
    }

    fn name(&self) -> String {
        "Equal".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an equality check".to_string()
    }

    fn signature(&self) -> String {
        "EQ $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "EQ".to_string()
    }

    fn opcode(&self) -> u8 {
        Equal::OPCODE
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

        let result = left == right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result as bool))
    }
}


pub struct NotEqual {
    operand_values: OperandValues,
}

impl NotEqual {
    pub const OPCODE: u8 = 21;
}

impl Instruction for NotEqual {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        NotEqual {
            operand_values
        }
    }

    fn name(&self) -> String {
        "NotEqual".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an != check".to_string()
    }

    fn signature(&self) -> String {
        "EQ $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "NEQ".to_string()
    }

    fn opcode(&self) -> u8 {
        NotEqual::OPCODE
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

        let result = left != right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result))
    }
}

pub struct GreaterThan {
    operand_values: OperandValues,
}

impl GreaterThan {
    pub const OPCODE: u8 = 22;
}

impl Instruction for GreaterThan {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        GreaterThan {
            operand_values
        }
    }

    fn name(&self) -> String {
        "GreaterThan".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an > check".to_string()
    }

    fn signature(&self) -> String {
        "GT $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "GT".to_string()
    }

    fn opcode(&self) -> u8 {
        GreaterThan::OPCODE
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

        let result = left > right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result))
    }
}

pub struct LessThan {
    operand_values: OperandValues,
}

impl LessThan {
    pub const OPCODE: u8 = 23;
}

impl Instruction for LessThan {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        LessThan {
            operand_values
        }
    }

    fn name(&self) -> String {
        "LessThan".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an < check".to_string()
    }

    fn signature(&self) -> String {
        "LT $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "LT".to_string()
    }

    fn opcode(&self) -> u8 {
        LessThan::OPCODE
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

        let result = left < right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result))
    }
}

pub struct GreaterThanOrEqual {
    operand_values: OperandValues,
}

impl GreaterThanOrEqual {
    pub const OPCODE: u8 = 24;
}

impl Instruction for GreaterThanOrEqual {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        GreaterThanOrEqual {
            operand_values
        }
    }

    fn name(&self) -> String {
        "GreaterThanOrEqual".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an >= check".to_string()
    }

    fn signature(&self) -> String {
        "GTE $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "GTE".to_string()
    }

    fn opcode(&self) -> u8 {
        GreaterThanOrEqual::OPCODE
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

        let result = left >= right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result))
    }
}

pub struct LessThanOrEqual {
    operand_values: OperandValues,
}

impl LessThanOrEqual {
    pub const OPCODE: u8 = 25;
}

impl Instruction for LessThanOrEqual {
    fn new(operand_values: OperandValues) -> Self where Self: Sized {
        LessThanOrEqual {
            operand_values
        }
    }

    fn name(&self) -> String {
        "LessThanOrEqual".to_string()
    }

    fn help(&self) -> String {
        "Store the result of an <= check".to_string()
    }

    fn signature(&self) -> String {
        "LTE $D $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "LTE".to_string()
    }

    fn opcode(&self) -> u8 {
        LessThanOrEqual::OPCODE
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

        let result = left <= right;

        vm.set_register(destination, result as RegisterValue).unwrap();
        Ok(ExecutionResult::Equality(result))
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::compare::{Equal, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, NotEqual};
    use crate::vm::Program;
    use crate::vm::VM;

    #[test]
    fn test_equal() {
        let program = Program::from([
            Equal::OPCODE, 30, 1, 2, // Pass
            Equal::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 100).unwrap();
        vm.set_register(2, 100).unwrap();
        vm.set_register(3, 200).unwrap();
        vm.set_register(4, 500).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(30).unwrap());
        assert_eq!(0, vm.register(31).unwrap());
    }

    #[test]
    fn test_not_equal() {
        let program = Program::from([
            NotEqual::OPCODE, 30, 1, 2, // Pass
            NotEqual::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 100).unwrap();
        vm.set_register(2, 200).unwrap();
        vm.set_register(3, 300).unwrap();
        vm.set_register(4, 300).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(30).unwrap());
        assert_eq!(0, vm.register(31).unwrap());
    }

    #[test]
    fn test_greater_than() {
        let program = Program::from([
            GreaterThan::OPCODE, 30, 1, 2, // Pass
            GreaterThan::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 300).unwrap();
        vm.set_register(2, 200).unwrap();
        vm.set_register(3, 200).unwrap();
        vm.set_register(4, 300).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(30).unwrap());
        assert_eq!(0, vm.register(31).unwrap());
    }

    #[test]
    fn test_less_than() {
        let program = Program::from([
            LessThan::OPCODE, 30, 1, 2, // Pass
            LessThan::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 100).unwrap();
        vm.set_register(2, 200).unwrap();
        vm.set_register(3, 400).unwrap();
        vm.set_register(4, 300).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(30).unwrap());
        assert_eq!(0, vm.register(31).unwrap());
    }

    #[test]
    fn test_greater_than_or_equal() {
        let program = Program::from([
            GreaterThanOrEqual::OPCODE, 28, 1, 2, // Pass
            GreaterThanOrEqual::OPCODE, 29, 3, 4, // Pass
            GreaterThanOrEqual::OPCODE, 30, 5, 6, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 200).unwrap();
        vm.set_register(2, 100).unwrap();

        vm.set_register(3, 200).unwrap();
        vm.set_register(4, 200).unwrap();

        vm.set_register(5, 200).unwrap();
        vm.set_register(6, 300).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(28).unwrap());
        assert_eq!(1, vm.register(29).unwrap());
        assert_eq!(0, vm.register(30).unwrap());
    }

    #[test]
    fn test_less_than_or_equal() {
        let program = Program::from([
            LessThanOrEqual::OPCODE, 28, 1, 2, // Pass
            LessThanOrEqual::OPCODE, 29, 3, 4, // Pass
            LessThanOrEqual::OPCODE, 30, 5, 6, // Fail
        ]);

        let mut vm = VM::new();
        vm.set_register(1, 100).unwrap();
        vm.set_register(2, 200).unwrap();

        vm.set_register(3, 200).unwrap();
        vm.set_register(4, 200).unwrap();

        vm.set_register(5, 400).unwrap();
        vm.set_register(6, 300).unwrap();

        vm.run(program);

        assert_eq!(1, vm.register(28).unwrap());
        assert_eq!(1, vm.register(29).unwrap());
        assert_eq!(0, vm.register(30).unwrap());
    }
}
