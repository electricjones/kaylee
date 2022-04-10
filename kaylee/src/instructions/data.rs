use std::fmt::Error;

use kaylee_derive::{Instruction, values};

use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee};

/// This is the help documentation for Load
/// And more documentation for Load
#[derive(Instruction)]
#[opcode = 1]
#[signature = "LOAD $D #[2]"]
pub struct Load {
    operand_values: OperandValues,
}

impl Executable for Load {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let destination = self.operand_value(0).unwrap().as_register_id();
        let value = self.operand_value(1).unwrap().as_constant_value();

        vm.set_register(destination, value).unwrap();
        Ok(ExecutionResult::Value(value))
    }
}


//
// pub struct Load {
//     operand_values: OperandValues,
// }
//
// impl Load {
//     pub const OPCODE: u8 = 1;
// }
//

//
// impl Instruction for Load {
//     fn new(operand_values: OperandValues) -> Self {
//         Load { operand_values }
//     }
//
//     fn signature() -> InstructionSignature where Self: Sized {
//         InstructionSignature {
//             identifier: "LOAD".to_string(),
//             operands: [OperandType::RegisterId, OperandType::ConstantHalfWord, OperandType::None],
//         }
//     }
//
//     fn documentation() -> InstructionDocumentation where Self: Sized {
//         InstructionDocumentation {
//             name: String::from("Load"),
//             help: String::from("Some help for Load"),
//         }
//     }
//
//     fn display(&self) -> String {
//         display_instruction_with_values(self)
//     }
//
//     fn operand_values(&self) -> &OperandValues {
//         &self.operand_values
//     }
// }

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