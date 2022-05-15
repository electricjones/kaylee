//! Instructions for arithmetic operations
//! Opcodes reserved: 70 - 99
use std::fmt::Error;

use kaylee_derive::Instruction;

use crate::instructions;
use crate::instructions::{display_instruction_with_values, Executable, Instruction, InstructionDocumentation, InstructionSignature, OperandType, OperandValues};
use crate::vm::{ExecutionResult, Kaylee, RegisterValue};

/// Add: Sums the value of two registers and loads the result into a third register
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///     - `RuntimeError`: If the result is too large for a destination register
///
/// Examples
/// ```asm
/// ADD $01 $10 $30 // `46 01 0A 1E` - Adds the value of register 10 to the value of register 30 ($10 + $30), and stores the result in register 1
/// ADD $40 $01 $10 // `46 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 70]
#[signature = "ADD $D $L $R"]
pub struct Add {
    operand_values: OperandValues,
}

impl Executable for Add {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left + right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        Ok(ExecutionResult::Value(result))
    }
}

/// Subtract: Subtracts the values of two registers and loads the result into a third register
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///     - `RuntimeError`: If the result is too large for a destination register
///
/// Examples
/// ```asm
/// SUB $01 $10 $30 // `47 01 0A 1E` - Subtracts the value of register 30 from the value of register 10 ($10 - $30), and stores the result in register 1
/// SUB $40 $01 $10 // `47 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 71]
#[signature = "SUB $D $L $R"]
pub struct Subtract {
    operand_values: OperandValues,
}

impl Executable for Subtract {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left - right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        Ok(ExecutionResult::Value(result))
    }
}

/// Multiply: Multiplies the values of two registers and loads the result into a third register
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///     - `RuntimeError`: If the result is too large for a destination register
///
/// Examples
/// ```asm
/// MUL $01 $10 $30 // `48 01 0A 1E` - Multiplies the value of register 10 and the value of register 30 ($10 * $30), and stores the result in register 1
/// MUL $40 $01 $10 // `48 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 72]
#[signature = "MUL $D $L $R"]
pub struct Multiply {
    operand_values: OperandValues,
}

impl Executable for Multiply {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
        let callback = |left: RegisterValue, right: RegisterValue| { (left * right) as RegisterValue };

        let result = instructions::basic_register_execution(self, vm, callback);
        Ok(ExecutionResult::Value(result))
    }
}

/// Divide: Divides the values of two registers, loads the result into a third register, and saves the remainder
/// Operands:
///     - 0: `$D` | 1 Byte | RegisterId | RegisterId of the destination register (0-31)
///     - 1: `$L` | 1 Byte | RegisterId | RegisterId of the left term
///     - 2: `$R` | 1 Byte | RegisterId | RegisterId of the right term
///
/// Errors/ Panics
///     - `AssemblerError` or `ProgramPanic`: If any register is out of bounds
///     - `RuntimeError`: If the result is too large for a destination register
///
/// Examples
/// ```asm
/// DIV $01 $10 $30 // `49 01 0A 1E` - Divides the values of registers 10 and 30 ($10 / $30), stores the result in register 1, with the remainder
/// DIV $40 $01 $10 // `48 28 01 0A` - AssemblerError because 40 is not a valid register
/// ```
#[derive(Instruction)]
#[opcode = 73]
#[signature = "DIV $D $L $R"]
pub struct Divide {
    operand_values: OperandValues,
}

impl Executable for Divide {
    fn execute(&self, vm: &mut Kaylee) -> Result<ExecutionResult, Error> {
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
    use crate::program::Program;
    use crate::vm::Kaylee;

    #[test]
    fn test_add() {
        let program = Program::from(vec![
            Add::OPCODE, 29, 0, 2,
            Add::OPCODE, 30, 1, 3,
            Add::OPCODE, 31, 29, 30,
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            Subtract::OPCODE, 29, 0, 2,
            Subtract::OPCODE, 30, 1, 3,
            Subtract::OPCODE, 31, 29, 30,
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            Multiply::OPCODE, 29, 0, 2,
            Multiply::OPCODE, 30, 1, 3,
            Multiply::OPCODE, 31, 29, 30,
        ]);

        let mut vm = Kaylee::new();
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
        let program = Program::from(vec![
            Divide::OPCODE, 31, 0, 1,
        ]);

        let mut vm = Kaylee::new();
        vm.set_register(0, 16).unwrap();
        vm.set_register(1, 2).unwrap();

        vm.run(program);

        assert_eq!(8, vm.register(31).unwrap());
        assert_eq!(0, vm.remainder());
    }

    #[test]
    fn test_divide_with_remainder() {
        let program = Program::from(vec![
            Divide::OPCODE, 31, 0, 1,
        ]);

        let mut vm = Kaylee::new();
        vm.set_register(0, 13).unwrap();
        vm.set_register(1, 5).unwrap();

        vm.run(program);

        assert_eq!(2, vm.register(31).unwrap());
        assert_eq!(3, vm.remainder());
    }

    #[test]
    fn test_math() {
        let program = Program::from(vec![
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

        let mut vm = Kaylee::new();
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
