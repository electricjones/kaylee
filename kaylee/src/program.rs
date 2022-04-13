use std::ops::Index;

use crate::vm::Byte;

pub type ProgramIndex = usize;

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

// impl IndexMut<ProgramIndex> for Program {
//     fn index_mut(&mut self, index: ProgramIndex) -> &mut Self::Output {
//         todo!()
//     }
// }