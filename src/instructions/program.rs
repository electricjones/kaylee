use std::fmt::Error;

use crate::instructions::{Instruction, OperandMap, OperandValues};
use crate::vm::{ExecutionResult, RegisterId, VM};

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
        OperandMap::from([3, 0, 0])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let destination = self.operand_values[0].as_program_index();

        vm.set_program_counter(destination);
        Ok(ExecutionResult::Jumped(destination))
    }
}

pub struct JumpForward {
    operand_values: OperandValues,
}

impl JumpForward {
    pub const OPCODE: u8 = 11;
}

impl Instruction for JumpForward {
    fn new(operand_values: OperandValues) -> Self {
        JumpForward { operand_values }
    }

    fn name(&self) -> String {
        "JumpForward".to_string()
    }

    fn help(&self) -> String {
        "Jumps forward some number of instructions".to_string()
    }

    fn signature(&self) -> String {
        "JMPF".to_string()
    }

    fn identifier(&self) -> String {
        "JMPF".to_string()
    }

    fn opcode(&self) -> u8 {
        JumpForward::OPCODE
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

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let forward = self.operand_values[0].as_constant_value();
        let steps = (forward * 4) as usize;

        vm.set_program_counter(vm.program_counter() + steps);
        Ok(ExecutionResult::Jumped(vm.program_counter()))
    }
}


pub struct JumpBackward {
    operand_values: OperandValues,
}

impl JumpBackward {
    pub const OPCODE: u8 = 12;
}

impl Instruction for JumpBackward {
    fn new(operand_values: OperandValues) -> Self {
        JumpBackward { operand_values }
    }

    fn name(&self) -> String {
        "JumpBackward".to_string()
    }

    fn help(&self) -> String {
        "Jumps bacwards some number of instructions".to_string()
    }

    fn signature(&self) -> String {
        "JMPB".to_string()
    }

    fn identifier(&self) -> String {
        "JMPB".to_string()
    }

    fn opcode(&self) -> u8 {
        JumpBackward::OPCODE
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

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let backward = self.operand_values[0].as_constant_value();
        let steps = ((backward + 1) * 4) as usize;

        vm.set_program_counter(vm.program_counter() - steps);
        Ok(ExecutionResult::Jumped(vm.program_counter()))
    }
}


pub struct JumpEqual {
    operand_values: OperandValues,
}

impl JumpEqual {
    pub const OPCODE: u8 = 15;
}

impl Instruction for JumpEqual {
    fn new(operand_values: OperandValues) -> Self {
        JumpEqual { operand_values }
    }

    fn name(&self) -> String {
        "JumpEqual".to_string()
    }

    fn help(&self) -> String {
        "Jumps to absolute if two values are equal".to_string()
    }

    fn signature(&self) -> String {
        "JMPE $JD $A $B".to_string()
    }

    fn identifier(&self) -> String {
        "JMPE".to_string()
    }

    fn opcode(&self) -> u8 {
        JumpEqual::OPCODE
    }

    fn operand_map() -> OperandMap {
        OperandMap::from([1, 1, 1])
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }

    fn set_operand_values(&mut self, operand_values: OperandValues) {
        self.operand_values = operand_values;
    }

    fn execute(&self, vm: &mut VM) -> Result<ExecutionResult, Error> {
        let destination = self.get_register_value_for_operand(0, vm).unwrap();
        let left = self.get_register_value_for_operand(1, vm).unwrap();
        let right = self.get_register_value_for_operand(2, vm).unwrap();

        if left == right {
            vm.set_program_counter(destination as RegisterId);
            return Ok(ExecutionResult::Jumped(vm.program_counter()));
        }

        Ok(ExecutionResult::NoAction)
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::data::Load;
    use crate::instructions::program::{Jump, JumpBackward, JumpEqual, JumpForward};
    use crate::instructions::system::Halt;
    use crate::vm::{Program, VM};

    #[test]
    fn test_jump() {
        let program = Program::from([
            // A bunch of random load instructions
            Load::OPCODE, 0, 0, 100,
            Load::OPCODE, 1, 0, 100,
            Jump::OPCODE, 0, 0, 24,
            Load::OPCODE, 2, 0, 100,
            Load::OPCODE, 3, 0, 100,
            Load::OPCODE, 4, 0, 100,
            Load::OPCODE, 5, 0, 100,
        ]);

        let mut vm = VM::new();
        vm.run(program);

        // Should set these
        assert_eq!(100, vm.register(0).unwrap());
        assert_eq!(100, vm.register(1).unwrap());

        // Should skip these
        assert_eq!(0, vm.register(2).unwrap());
        assert_eq!(0, vm.register(3).unwrap());
        assert_eq!(0, vm.register(4).unwrap());

        // And hit this one at the end
        assert_eq!(100, vm.register(5).unwrap());

        // And check on the counter itself
        assert_eq!(28, vm.program_counter());
    }

    #[test]
    fn test_jump_forward() {
        let program = Program::from([
            // A bunch of random load instructions
            Load::OPCODE, 0, 0, 100,
            Load::OPCODE, 1, 0, 100,
            JumpForward::OPCODE, 0, 0, 3,
            Load::OPCODE, 2, 0, 100,
            Load::OPCODE, 3, 0, 100,
            Load::OPCODE, 4, 0, 100,
            Load::OPCODE, 5, 0, 100,
        ]);

        let mut vm = VM::new();
        vm.run(program);

        // Should set these
        assert_eq!(100, vm.register(0).unwrap());
        assert_eq!(100, vm.register(1).unwrap());

        // Should skip these
        assert_eq!(0, vm.register(2).unwrap());
        assert_eq!(0, vm.register(3).unwrap());
        assert_eq!(0, vm.register(4).unwrap());

        // And hit this one at the end
        assert_eq!(100, vm.register(5).unwrap());

        // And check on the counter itself
        assert_eq!(28, vm.program_counter());
    }

    #[test]
    fn test_jump_backward() {
        let program = Program::from([
            Load::OPCODE, 0, 0, 100, // Jump to here, execute

            Halt::OPCODE, 0, 0, 0, // Stop

            Load::OPCODE, 1, 0, 100,
            Load::OPCODE, 2, 0, 100, // Start here (12)
            Load::OPCODE, 3, 0, 100, // Execute

            JumpBackward::OPCODE, 0, 0, 5, // Jump

            Load::OPCODE, 4, 0, 100,
            Load::OPCODE, 5, 0, 100,
        ]);

        let mut vm = VM::new();
        vm.set_program_counter(12);
        vm.run(program);

        assert_eq!(100, vm.register(0).unwrap());
        assert_eq!(0, vm.register(1).unwrap());
        assert_eq!(100, vm.register(2).unwrap());
        assert_eq!(100, vm.register(3).unwrap());
        assert_eq!(0, vm.register(4).unwrap());
        assert_eq!(0, vm.register(5).unwrap());

        // And check on the counter itself
        assert_eq!(8, vm.program_counter());
    }

    #[test]
    fn test_jump_if_equal() {
        let program = Program::from([
            // A bunch of random load instructions
            Load::OPCODE, 0, 0, 100,
            Load::OPCODE, 1, 0, 100,
            JumpEqual::OPCODE, 30, 29, 28,
            Load::OPCODE, 2, 0, 100,
            Load::OPCODE, 3, 0, 100,
            Load::OPCODE, 4, 0, 100,
            Load::OPCODE, 5, 0, 100,
        ]);

        let mut vm = VM::new();
        vm.set_register(30, 24).unwrap();
        vm.set_register(29, 200).unwrap();
        vm.set_register(28, 200).unwrap();

        vm.run(program);

        // Should set these
        assert_eq!(100, vm.register(0).unwrap());
        assert_eq!(100, vm.register(1).unwrap());

        // Should skip these
        assert_eq!(0, vm.register(2).unwrap());
        assert_eq!(0, vm.register(3).unwrap());
        assert_eq!(0, vm.register(4).unwrap());

        // And hit this one at the end
        assert_eq!(100, vm.register(5).unwrap());

        // And check on the counter itself
        assert_eq!(28, vm.program_counter());
    }

    #[test]
    fn test_dont_jump_if_not_equal() {
        let program = Program::from([
            // A bunch of random load instructions
            Load::OPCODE, 0, 0, 100,
            Load::OPCODE, 1, 0, 100,
            JumpEqual::OPCODE, 30, 29, 28,
            Load::OPCODE, 2, 0, 100,
            Load::OPCODE, 3, 0, 100,
            Load::OPCODE, 4, 0, 100,
            Load::OPCODE, 5, 0, 100,
        ]);

        let mut vm = VM::new();
        vm.set_register(30, 24).unwrap();
        vm.set_register(29, 300).unwrap();
        vm.set_register(28, 200).unwrap();

        vm.run(program);

        // Should set these
        assert_eq!(100, vm.register(0).unwrap());
        assert_eq!(100, vm.register(1).unwrap());
        assert_eq!(100, vm.register(2).unwrap());
        assert_eq!(100, vm.register(3).unwrap());
        assert_eq!(100, vm.register(4).unwrap());
        assert_eq!(100, vm.register(5).unwrap());

        // And check on the counter itself
        assert_eq!(28, vm.program_counter());
    }
}