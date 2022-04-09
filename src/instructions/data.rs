use std::fmt::Error;

use crate::instructions::{display_instruction_with_values, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee};

// #[derive(KayleeInstruction)]
// #[Opcode(100)]
// #[Signature("LOAD $D #[2]")]
// #[Signature("LOAD $D $A #[1]")]
// #[Signature("ADD $D $A $B")]
// #[Signature("JUMP #[3]")]
/// This would be the help documentation
/// https://stackoverflow.com/questions/33999341/generating-documentation-in-macros
// @todo: separate Executor from Instruction so Instruction can be derived and Executor can be implemented

// pub struct Load {}
//
// impl Executable for Load {
//     fn execute(&self, &vm: Kaylee) {
//
//     }
// }
//
pub struct Load {
    operand_values: OperandValues,
}

impl Load {
    pub const OPCODE: u8 = 1;
}

impl Instruction for Load {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let destination = self.operand_values[0].as_register_id();
        let value = self.operand_values[1].as_constant_value();

        vm.set_register(destination, value).unwrap();
        Ok(ExecutionResult::Value(value))
    }

    fn new(operand_values: OperandValues) -> Self {
        Load { operand_values }
    }

    fn signature() -> InstructionSignature where Self: Sized {
        InstructionSignature {
            identifier: "LOAD".to_string(),
            operands: [OperandType::RegisterId, OperandType::ConstantHalfWord, OperandType::None],
        }
    }

    fn documentation() -> InstructionDocumentation where Self: Sized {
        InstructionDocumentation {
            name: String::from("Load"),
            help: String::from("Some help for Load"),
        }
    }

    fn display(&self) -> String {
        display_instruction_with_values(self)
    }

    fn operand_values(&self) -> &OperandValues {
        &self.operand_values
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::Kaylee;
    use crate::vm::Program;

    #[test]
    fn test_load() {
        let program = Program::from([
            1, 4, 1, 244, // LOAD $4 #500
            1, 30, 0, 12,  // LOAD $6 #12
        ]);

        let mut vm = Kaylee::new();
        vm.run(program);

        assert_eq!(500, vm.register(4).unwrap());
        assert_eq!(12, vm.register(30).unwrap());
    }
}