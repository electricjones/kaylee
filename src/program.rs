pub type ProgramIndex = usize;

pub type Program = Vec<u8>;

// pub struct Program {
//     // @todo: I'm torn between the counter being part of the program or the vm
//     // As part of the program, this iterator works nicely
//     // But it also means my program is at least somewhat mutable. No bueno.
//     // counter: usize,
//     instructions: Vec<u8>,
// }
//
// impl Program {
//     pub fn new(instructions: Vec<u8>) -> Self {
//         Program {
//             // counter: 0,
//             instructions,
//         }
//     }
//
//     pub(crate) fn extend(&mut self, bytes: Vec<u8>) {
//         self.instructions.extend(bytes);
//     }
//
//     // pub fn counter(&self) -> ProgramIndex {
//     //     self.counter
//     // }
//     //
//     // pub fn set_counter(&mut self, index: ProgramIndex) {
//     //     self.counter = index;
//     // }
// }

// impl Iterator for Program {
//     type Item = Box<dyn Instruction>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if &self.counter >= &self.instructions.len() {
//             return None;
//         }
//
//         Some(crate::instructions::decode_next_instruction(&mut self.instructions, &mut self.counter).unwrap())
//     }
// }
