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


#[cfg(test)]
mod tests {
    use crate::instructions::data::Load;
    use crate::instructions::program::Jump;
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
}