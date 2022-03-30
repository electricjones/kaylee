use crate::instructions::Instruction;

pub struct Program {
    counter: usize,
    instructions: Vec<u8>,
}

impl Program {
    pub fn new(instructions: Vec<u8>) -> Self {
        Program {
            counter: 0,
            instructions,
        }
    }

    pub(crate) fn extend(&mut self, bytes: Vec<u8>) {
        self.instructions.extend(bytes);
    }
}

impl Iterator for Program {
    type Item = Box<dyn Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.counter >= &self.instructions.len() {
            return None;
        }

        Some(crate::instructions::decode_next_instruction(&mut self.instructions, &mut self.counter).unwrap())
        // *&mut self.counter += 1;
        // Some(instruction)
    }
}
