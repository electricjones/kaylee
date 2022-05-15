use std::ops::Index;

use crate::asm::{Parsed, Source};
use crate::asm::assembler::{Assembler, AssemblerError};
use crate::asm::parser::parse_asm;
use crate::vm::Byte;

pub type ProgramIndex = usize;

#[derive(PartialEq, Debug)]
pub struct Program {
    bytes: Vec<Byte>,
}

impl Index<ProgramIndex> for Program {
    type Output = Byte;

    fn index(&self, index: ProgramIndex) -> &Self::Output {
        &self.bytes[index]
    }
}

impl From<Vec<Byte>> for Program {
    fn from(bytes: Vec<Byte>) -> Self {
        Program { bytes }
    }
}

impl Extend<Byte> for Program {
    fn extend<T: IntoIterator<Item=Byte>>(&mut self, iter: T) {
        self.bytes.extend(iter)
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            bytes: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl<'a> TryFrom<Parsed<'a>> for Program {
    type Error = AssemblerError;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        let assembler = Assembler::new();
        assembler.assemble_parsed_asm(parsed)
    }
}

impl TryFrom<Source> for Program {
    type Error = AssemblerError;

    fn try_from(source: Source) -> Result<Self, Self::Error> {
        // let parsed = Parsed::try_from(source);
        let parsed = parse_asm(source.body.as_str());
        match parsed {
            Ok(success) => {
                success.1.try_into()
            }
            Err(_) => Err(AssemblerError::Other(String::from("Parsing error")))
        }
    }
}

// impl IndexMut<ProgramIndex> for Program {
//     fn index_mut(&mut self, index: ProgramIndex) -> &mut Self::Output {
//         todo!()
//     }
// }