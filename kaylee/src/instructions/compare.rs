//! Instructions for comparisons
//! Opcodes reserved: 100 - 119
use std::fmt::Error;

use kaylee_derive::Instruction;

use crate::instructions;
use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee, RegisterValue};

/// Equal: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// EQ $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// EQ $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 110]
#[signature = "EQ $D $L $R"]
pub struct Equal {
    operand_values: OperandValues,
}

impl Executable for Equal {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left == right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

/// NotEqual: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// NEQ $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// NEQ $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 111]
#[signature = "NEQ $D $L $R"]
pub struct NotEqual {
    operand_values: OperandValues,
}

impl Executable for NotEqual {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left != right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

/// GreaterThan: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// GT $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// GT $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 112]
#[signature = "GT $D $L $R"]
pub struct GreaterThan {
    operand_values: OperandValues,
}

impl Executable for GreaterThan {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left > right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

/// LessThan: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// LT $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// LT $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 113]
#[signature = "LT $D $L $R"]
pub struct LessThan {
    operand_values: OperandValues,
}

impl Executable for LessThan {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left < right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

/// GreaterThanOrEqual: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// GTE $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// GTE $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 114]
#[signature = "GTE $D $L $R"]
pub struct GreaterThanOrEqual {
    operand_values: OperandValues,
}

impl Executable for GreaterThanOrEqual {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left >= right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

/// LessThanOrEqual: Stores a boolean in a destination with the comparison result from two register values
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///
/// Examples
/// ```asm
/// LTE $01 $10 $30 // `6E 01 0A 1E` - Loads true/false into register 1 based on comparison from values in registers 10 and 30
/// LTE $40 $01 $10 // `6E 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 115]
#[signature = "LTE $D $L $R"]
pub struct LessThanOrEqual {
    operand_values: OperandValues,
}

impl Executable for LessThanOrEqual {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left <= right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        match result {
            1 => Ok(ExecutionResult::Equality(true)),
            0 => Ok(ExecutionResult::Equality(false)),
            _ => panic!("Equality returned something other than a 0 or 1")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::compare::{Equal, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, NotEqual};
    use crate::program::Program;
    use crate::vm::Kaylee;

    #[test]
    fn test_equal() {
        let program = Program::from(vec![
            Equal::OPCODE, 30, 1, 2, // Pass
            Equal::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            NotEqual::OPCODE, 30, 1, 2, // Pass
            NotEqual::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            GreaterThan::OPCODE, 30, 1, 2, // Pass
            GreaterThan::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            LessThan::OPCODE, 30, 1, 2, // Pass
            LessThan::OPCODE, 31, 3, 4, // Fail
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            GreaterThanOrEqual::OPCODE, 28, 1, 2, // Pass
            GreaterThanOrEqual::OPCODE, 29, 3, 4, // Pass
            GreaterThanOrEqual::OPCODE, 30, 5, 6, // Fail
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            LessThanOrEqual::OPCODE, 28, 1, 2, // Pass
            LessThanOrEqual::OPCODE, 29, 3, 4, // Pass
            LessThanOrEqual::OPCODE, 30, 5, 6, // Fail
        ]);

        let mut vm = Kaylee::new();
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
