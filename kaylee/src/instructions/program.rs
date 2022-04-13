//! Instructions for navigating and manipulating the program
//! Opcodes reserved: 50 - 69
use std::fmt::Error;

use kaylee_derive::Instruction;

use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee, RegisterId};

/// Jump: Resets the program counter to a constant value
/// Operands:
///     - 0: `#ADDRESS` | 3 Bytes | ProgramIndex | ProgramIndex to jump to
///
/// Errors/ Panics
///     - `AssemblerError`: If the ProgramIndex is a value larger than 3 bytes
///     - `RuntimeError`: If the target ProgramIndex is out of bounds
///
/// Examples
/// ```asm
/// JUMP #500 // `32 00 01 FF` - Jumps to program index 500
/// ```
#[derive(Instruction)]
#[opcode = 50]
#[signature = "JUMP #3"]
pub struct Jump {
    operand_values: OperandValues,
}

impl Executable for Jump {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let destination = self.operand_values[0].as_program_index();

        vm.set_program_counter(destination);
        Ok(ExecutionResult::Jumped(destination))
    }
}

/// JumpForward: Moves the program forward a certain number of instructions
/// Operands:
///     - 0: `#NUM_OF_INSTRUCTIONS` | 3 Bytes | ConstantValue | Number of instructions to move forward
///
/// Errors/ Panics
///     - `AssemblerError`: If the ConstantValue is a value larger than 3 bytes
///     - `RuntimeError`: If the target ProgramIndex is out of bounds
///
/// Examples
/// ```asm
/// JUMPF #4 // `33 00 01 FF` - Jumps forward 4 instructions (16 bytes)
/// ```
#[derive(Instruction)]
#[opcode = 51]
#[signature = "JUMPF #3"]
pub struct JumpForward {
    operand_values: OperandValues,
}

impl Executable for JumpForward {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let forward = self.operand_values[0].as_constant_value();
        let steps = (forward * 4) as usize;

        vm.set_program_counter(vm.program_counter() + steps);
        Ok(ExecutionResult::Jumped(vm.program_counter()))
    }
}

/// JumpBackward: Moves the program backward a certain number of instructions
/// Operands:
///     - 0: `#NUM_OF_INSTRUCTIONS` | 3 Bytes | ConstantValue | Number of instructions to move forward
///
/// Errors/ Panics
///     - `AssemblerError`: If the ConstantValue is a value larger than 3 bytes
///     - `RuntimeError`: If the target ProgramIndex is out of bounds
///
/// Examples
/// ```asm
/// JUMPF #4 // `34 00 01 FF` - Jumps backward 4 instructions (16 bytes)
/// ```
#[derive(Instruction)]
#[opcode = 52]
#[signature = "JUMPB #3"]
pub struct JumpBackward {
    operand_values: OperandValues,
}

impl Executable for JumpBackward {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let backward = self.operand_values[0].as_constant_value();
        let steps = ((backward + 1) * 4) as usize;

        vm.set_program_counter(vm.program_counter() - steps);
        Ok(ExecutionResult::Jumped(vm.program_counter()))
    }
}

/// JumpEqual: Moves the program counter to the value of a register if the value of two registers is equal
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId that holds the target ProgramIndex
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError`: If any RegisterIds are out of bounds
///     - `RuntimeError`: If the target ProgramIndex is out of bounds
///
/// Examples
/// ```asm
/// JUMPE $0 $1 $2 // `34 00 01 02` - Jumps to the value of R1 if R2 and R3 are equal
/// ```
#[derive(Instruction)]
#[opcode = 53]
#[signature = "JUMPE $D $L $R"]
pub struct JumpEqual {
    operand_values: OperandValues,
}

impl Executable for JumpEqual {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
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
    use crate::instructions::machine::Halt;
    use crate::instructions::program::{Jump, JumpBackward, JumpEqual, JumpForward};
    use crate::program::Program;
    use crate::vm::Kaylee;

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

        let mut vm = Kaylee::new();
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

        let mut vm = Kaylee::new();
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

        let mut vm = Kaylee::new();
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

        let mut vm = Kaylee::new();
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

        let mut vm = Kaylee::new();
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