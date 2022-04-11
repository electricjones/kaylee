use std::fmt::Error;

use kaylee_derive::Instruction;

use crate::instructions;
use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee, RegisterId, RegisterValue};

/// Halt: Gracefully ends the program and shuts down the process
/// Operands:
///     - None
///
/// Errors/ Panics
///     - None
///
/// Examples
/// ```asm
/// HALT // `01 00 00 00`
/// ```
#[derive(Instruction)]
#[opcode = 1]
#[signature = "HALT"]
pub struct Halt {
    operand_values: OperandValues,
}

impl Executable for Halt {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        vm.halt();
        Ok(ExecutionResult::Halted)
    }
}