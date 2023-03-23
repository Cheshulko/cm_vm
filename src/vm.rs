use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32], // 32 = 8bits opcode + 8 register number + 2*8 bits
    pc: usize,            // program counter that tracks which byte is being executed
    program: Vec<u8>,     // Bytecode of the program. 8bits for opcode
    remainder: u32,       // Modulo for Opcode::DIV
    equal_flag: bool,     // Result of the last comparison operation
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    // 8bits -> opcode
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        // program[i] = 11010000
        // program[i+1] = 11110011
        // result should be 1101000011110011 (16bits)
        // TODO: 8 to const.
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    fn execute_instruction(&mut self) -> bool /* Is done */ {
        if self.pc >= self.program.len() {
            return true;
        }
        let decoded_opcode = self.decode_opcode();
        println!("decoded_opcode: {:?}", decoded_opcode);
        match decoded_opcode {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
                return false;
            }
            //
            Opcode::ADD => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 + value2;
                return false;
            }
            Opcode::SUB => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 - value2;
                return false;
            }
            Opcode::MUL => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = value1 * value2;
                return false;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32; // TODO: u32 ????
                return false;
            }
            //
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
                // TODO: `Eat 8+8bits ??`
                return false;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                println!("{} {}", self.pc, value);
                self.pc += value as usize;
                // TODO: `Eat 8+8bits ??`
                return false;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
                // TODO: `Eat 8+8bits ??`
                return false;
            }
            //
            Opcode::EQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 == value2;
                self.next_8_bits();
                return false;
            }
            Opcode::NEQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 != value2;
                self.next_8_bits();
                return false;
            }
            Opcode::GT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 > value2;
                self.next_8_bits();
                return false;
            }
            Opcode::LT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 < value2;
                self.next_8_bits();
                return false;
            }
            Opcode::GTQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 >= value2;
                self.next_8_bits();
                return false;
            }
            Opcode::LTQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = value1 <= value2;
                self.next_8_bits();
                return false;
            }
            //
            Opcode::JEQ => {
                let value = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = value as usize;
                }
                // TODO: `Eat 8+8bits ??`
                return false;
            }
            //
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
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
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244]; // 1 << 8 + 244 = 2^8 + 244 = 500
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.pc = 4;
        test_vm.program = vec![6, 0, 0, 0, 8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 15;
        test_vm.registers[1] = 11;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 15;
        test_vm.registers[1] = 19;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 15;
        test_vm.registers[1] = 15;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 15;
        test_vm.registers[1] = 15;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}