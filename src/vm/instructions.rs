// use std::fmt;
// use std::fmt::Formatter;
//
// #[derive(Debug, PartialEq)]
// pub enum Opcode {
//     HALT,
//     ILLEGAL,
//     LOAD,
//     ADD,
//     SUBTRACT,
//     MULTIPLY,
//     DIVIDE,
//     JUMP,
//     JUMP_FORWARD,
//     JUMP_BACKWARD,
//     EQ,
//     NEQ,
//     GT,
//     LT,
//     GTE,
//     LTE,
//     JEQ,
//     // @todo: these should all be CamelCase
// }
//
// // @todo: encode the long, short, code, and hex into the actual variant in one place
// // @todo: Opcode::Halt() -> HALT("HALT", "HLT", "0x02", 2)
//
// impl fmt::Display for Opcode {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match *self {
//             Opcode::HALT => write!(f, "HALT [HLT][0x00]"),
//             Opcode::ILLEGAL => write!(f, "ILLEGAL [IGL][0x02"),
//             Opcode::LOAD => writeln!(f, "LOAD [LOAD][0x03]"),
//             Opcode::ADD => writeln!(f, "ADD [ADD][0x04]"),
//             Opcode::SUBTRACT => writeln!(f, "SUBRACT [SUB][0x05]"),
//             Opcode::MULTIPLY => writeln!(f, "MULTIPLY [MUL][0x06]"),
//             Opcode::DIVIDE => writeln!(f, "DIVIDE [DIV][0x07]"),
//             Opcode::JUMP => writeln!(f, "JUMP [JMP][0x07]"),
//             Opcode::JUMP_FORWARD => writeln!(f, "JUMP_FORWARD [JMPF][0x07]"), // @todo: these should be instruction sizes
//             Opcode::JUMP_BACKWARD => writeln!(f, "JUMP_BACKWARD [JMPB][0x07]"),
//             _ => { writeln!(f, "equality") }
//         }
//     }
// }
//
// impl From<u8> for Opcode {
//     fn from(byte: u8) -> Self {
//         match byte {
//             0 => Opcode::HALT,
//             1 => Opcode::LOAD,
//             2 => Opcode::ADD,
//             3 => Opcode::SUBTRACT,
//             4 => Opcode::MULTIPLY,
//             5 => Opcode::DIVIDE,
//             6 => Opcode::JUMP,
//             7 => Opcode::JUMP_FORWARD,
//             8 => Opcode::JUMP_BACKWARD,
//             9 => Opcode::EQ,
//             10 => Opcode::NEQ,
//             11 => Opcode::GT,
//             12 => Opcode::LT,
//             13 => Opcode::GTE,
//             14 => Opcode::LTE,
//             15 => Opcode::JEQ,
//             _ => Opcode::ILLEGAL,
//         }
//     }
// }
//
// pub struct Instruction2 {
//     opcode: Opcode,
// }
//
// impl Instruction2 {
//     pub fn new(opcode: Opcode) -> Instruction2 {
//         Instruction2 {
//             opcode
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_create_hlt() {
//         let opcode = Opcode::HALT;
//         assert_eq!(opcode, Opcode::HALT)
//     }
//
//     #[test]
//     fn test_create_instruction() {
//         let instruction = Instruction2::new(Opcode::HALT);
//         assert_eq!(instruction.opcode, Opcode::HALT)
//     }
// }