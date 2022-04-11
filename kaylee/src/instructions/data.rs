//! Instructions for manipulating data (registers and memory)
//! Opcodes reserved: 30 - 49
use std::fmt::Error;

use kaylee_derive::{Instruction, values};

use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee};

/// LOAD: Loads a value into a designated register
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `#2` | 2 Bytes | HalfWord | Literal value to be loaded
///     - 2: NOT USED, given to Operand 1
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If register is out of bounds
///     - `AssemblerError` or `ProgramPanic`: If Constant value is too large for 2 bytes
///
/// Examples
/// ```asm
/// LOAD $1 #500 // `1E 01 01 FF` - Loads 500 into Register 1
/// LOAD $31 #01 // `1E 1F 00 01` - Loads 1 into Register 31
/// LOAD $40 #10 // `1E 28 00 0A` - Assembler Error because 40 is not a valid register
/// LOAD $15 #1,000,000 // `1E 0F 00 0A` - Assembler Error because constant value is out of bounds
/// ```
#[derive(Instruction)]
#[opcode = 30]
#[signature = "LOAD $D #2"]
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

#[cfg(test)]
mod tests {
    use crate::instructions::data::Load;
    use crate::vm::Kaylee;
    use crate::vm::Program;

    #[test]
    fn test_load() {
        let program = Program::from([
            Load::OPCODE, 4, 1, 244,  // LOAD $4 #500
            Load::OPCODE, 30, 0, 12,  // LOAD $6 #12
        ]);

        let mut vm = Kaylee::new();
        vm.run(program);

        assert_eq!(500, vm.register(4).unwrap());
        assert_eq!(12, vm.register(30).unwrap());
    }
}