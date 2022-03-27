use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HALT,
    ILLEGAL,
    LOAD,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

// @todo: encode the long, short, code, and hex into the actual variant in one place
// @todo: Opcode::Halt() -> HALT("HALT", "HLT", "0x02", 2)

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Opcode::HALT => write!(f, "HALT [HLT][0x00]"),
            Opcode::ILLEGAL => write!(f, "ILLEGAL [IGL][0x02"),
            Opcode::LOAD => writeln!(f, "LOAD [LOAD][0x03]"),
            Opcode::ADD => writeln!(f, "ADD [ADD][0x04]"),
            Opcode::SUBTRACT => writeln!(f, "SUBRACT [SUB][0x05]"),
            Opcode::MULTIPLY => writeln!(f, "MULTIPLY [MUL][0x06]"),
            Opcode::DIVIDE => writeln!(f, "DIVIDE [DIV][0x07]"),
        }
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::HALT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUBTRACT,
            4 => Opcode::MULTIPLY,
            5 => Opcode::DIVIDE,
            _ => Opcode::ILLEGAL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT)
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HALT);
        assert_eq!(instruction.opcode, Opcode::HALT)
    }
}