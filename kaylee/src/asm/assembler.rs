use linkme::distributed_slice;

use crate::instructions::OperandType;
use crate::program::Program;

pub type MapOperands = (&'static str, u8, [OperandType; 3]);

// @todo: This probably belongs elsewhere. In instructions, I'd wager.
#[distributed_slice]
pub static MY_MAP: [MapOperands] = [..];

#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    Other(String),
}

pub struct Assembler {
    // There will be state needed eventually
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {}
    }

    /// @todo: This is awful. Absolutely no error checking
    pub fn assemble_parsed_asm(&self, parsed: Vec<Vec<&str>>) -> Result<Program, AssemblerError> {
        let mut bytes: Vec<u8> = Vec::new();

        for instruction in parsed {
            // Get the info from the hashmap
            let mut item: Option<&MapOperands> = None;
            for a in MY_MAP {
                if a.0 == instruction[0] {
                    item = Some(a);
                    break;
                }
            }

            // Push the opcode
            bytes.push(item.unwrap().1.clone());

            for i in 1..(instruction.len()) {
                if let Some(value) = instruction.get(i) {
                    // this is an operand, so we have to break it into u8 chunks
                    let number = value.parse::<i32>().unwrap();
                    let operand_bytes = number.to_be_bytes();

                    let spot: &OperandType = &item.unwrap().2[i - 1];

                    let byte_count = match spot {
                        OperandType::None => 0 as u8,
                        OperandType::RegisterId => 1 as u8,
                        OperandType::ConstantByte => 1 as u8,
                        OperandType::ConstantHalfWord => 2 as u8,
                        OperandType::ConstantWord => 3 as u8,
                    };

                    let start_slice = (4 - byte_count) as usize;

                    bytes.extend(&operand_bytes[start_slice..]);
                }
            }
        }

        Ok(Program::from(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::assembler::Assembler;
    use crate::program::Program;

    #[test]
    pub fn test_into_bytecode() {
        let parsed = vec![
            vec!["LOAD", "1", "500"],
            vec!["ADD", "2", "3", "2"],
        ];

        let expected = Program::from(vec![
            30, 1, 1, 244,
            70, 2, 3, 2,
        ]);

        let assembler = Assembler::new();
        let actual = assembler.assemble_parsed_asm(parsed).unwrap();

        assert_eq!(expected, actual);
    }
}
