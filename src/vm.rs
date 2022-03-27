use crate::vm::instructions::Opcode;

mod instructions;

pub struct VM {
    pub(crate) registers: [i32; 32],
    counter: usize,
    pub instructions: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            counter: 0,
            instructions: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.counter >= self.instructions.len() {
            return true;
        }

        match self.decode__next_opcode() {
            Opcode::HALT => {
                println!("Encountered `{:?}`", Opcode::HALT);
                true
            }
            Opcode::ILLEGAL => {
                println!("Encountered `{:?}`", Opcode::ILLEGAL);
                true
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
                false
            }
            Opcode::ADD => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = left + right;
                false
            }
            Opcode::SUBTRACT => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = left - right;
                false
            }
            Opcode::MULTIPLY => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = left * right;
                false
            }
            Opcode::DIVIDE => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = left / right;
                self.remainder = (left % right) as u32;
                false
            }
            Opcode::JUMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.counter = target as usize;
                false
            }
            Opcode::JUMP_FORWARD => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.counter += value;
                false
            }
            Opcode::JUMP_BACKWARD => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.counter -= value;
                false
            }
            Opcode::EQ => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left == right) as i32;
                false
            }
            Opcode::NEQ => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left != right) as i32;
                false
            }
            Opcode::GT => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left > right) as i32;
                false
            }
            Opcode::LT => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left < right) as i32;
                false
            }
            Opcode::GTE => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left >= right) as i32;
                false
            }
            Opcode::LTE => {
                let destination = self.next_8_bits() as usize;
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                self.registers[destination] = (left <= right) as i32;
                false
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                let left = self.registers[self.next_8_bits() as usize];
                let right = self.registers[self.next_8_bits() as usize];

                if left == right {
                    self.counter = target as usize;
                }

                false
            }
        }
    }

    // @todo: this shouldn't be mutable
    // @todo: next_instruction() should be mutable
    fn decode__next_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.instructions[self.counter]);
        self.counter += 1;
        opcode
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.instructions[self.counter];
        self.counter += 1;
        result
    }
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.instructions[self.counter] as u16) << 8) | self.instructions[self.counter + 1] as u16;
        self.counter += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_halt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.instructions = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_opcode_illegal() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.instructions = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 4, // LOAD $1 #4
            1, 2, 0, 5, // LOAD $2 #5
            2, 0, 1, 2, // ADD $0 $1 $2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 9);
    }

    #[test]
    fn test_subtract() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 10, // LOAD $1 #4
            1, 2, 0, 6,  // LOAD $2 #5
            3, 0, 1, 2,  // SUB $0 $1 $2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 4);
    }

    #[test]
    fn test_multiply() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 2, // LOAD $1 #2
            1, 2, 0, 3,  // LOAD $2 #3
            4, 0, 1, 2,  // MUL $0 $1 $2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 6);
    }

    #[test]
    fn test_divide_no_remainder() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 6, // LOAD $1 #2
            1, 2, 0, 2,  // LOAD $2 #3
            5, 0, 1, 2,  // DIV $0 $1 $2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 3);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_divide_with_remainder() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 6, // LOAD $1 #2
            1, 2, 0, 4,  // LOAD $2 #3
            5, 0, 1, 2,  // DIV $0 $1 $2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.remainder, 2);
    }

    #[test]
    fn test_math() {
        let mut test_vm = VM::new();
        test_vm.instructions = vec![
            1, 1, 0, 6, // LOAD $1 #2
            1, 2, 0, 4,  // LOAD $2 #3
            2, 0, 1, 2,  // ADD $0 $1 $2
            3, 3, 0, 2,  // SUB $3 $0 $2
            4, 4, 1, 2,  // MUL
            5, 5, 0, 1,  // DIV
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 6);
        assert_eq!(test_vm.registers[2], 4);
        assert_eq!(test_vm.registers[3], 6);
        assert_eq!(test_vm.registers[4], 24);
        assert_eq!(test_vm.registers[5], 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.instructions = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.instructions = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.counter, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.instructions = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.counter, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.instructions = vec![9, 2, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);

        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 3;
        test_vm.instructions = vec![9, 2, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 10;
        test_vm.instructions = vec![15, 2, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.counter, 10);

        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 3;
        test_vm.registers[2] = 10;
        test_vm.instructions = vec![9, 2, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.counter, 4);
    }
}