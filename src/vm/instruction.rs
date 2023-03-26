#[derive(Debug, PartialEq, Eq, Copy, Clone)]
// 8bits opcode
pub enum Opcode {
    LOAD, // `LOAD $0 #500` Load value into register $0. #500 16 bits
    //
    ADD, // `ADD $0 $1 $2` add $0 + $1 registers. Save to $2 register
    SUB, // `SUB $0 $1 $2` sub $0 - $1 registers. Save to $2 register
    MUL, // `MUL $0 $1 $2` mul $0 * $1 registers. Save to $2 register
    DIV, // `DIV $0 $1 $2` mul $0 / $1 registers. Save to $2 register. Use `remainder`
    //
    HLT, // Stop execution
    //
    JMP,  // `JMP $0`.  Jump to $0 program byte.  Absolute jump // TODO: Should it be 18bits ?
    JMPF, // `JMPF $0`  Jump forwards by $0.      Relative jump // TODO: Should it be 18bits ?
    JMPB, // `JMPB $0`  Jump backwards by $0.     Relative jump // TODO: Should it be 18bits ?
    //
    EQ,  // `EQ  $0 $1 $unused`  aka ($0 == $1) Save result to `equal_flag`. Equal
    NEQ, // `NEQ $0 $1 $unused`  aka ($0 != $1) Save result to `equal_flag`. Not equal
    GT,  // `GT  $0 $1 $unused`  aka ($0 >  $1) Save result to `equal_flag`. Greater than
    LT,  // `LT  $0 $1 $unused`  aka ($0 <  $1) Save result to `equal_flag`. Less than
    GTQ, // `GTQ $0 $1 $unused`  aka ($0 >= $1) Save result to `equal_flag`. Greater than OR equal to
    LTQ, // `LTQ $0 $1 $unused`  aka ($0 <= $1) Save result to `equal_flag`. Less than OR equal to
    //
    JEQ, // `JEQ $0` Jump to $0 if equal (`equal_flag` is true)  Absolute jump // TODO: Should it be 18bits ?
    // TODO: JNEQ ???
    ALOC, // `ALOC $0` Allocate $0 bytes of memory in the heap
    //
    IGL, // Illegal
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            //
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            //
            5 => Opcode::HLT,
            //
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            //
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            //
            15 => Opcode::ALOC,
            //
            16 => Opcode::JEQ,
            //
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
