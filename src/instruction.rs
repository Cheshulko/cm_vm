#[derive(Debug, PartialEq)]
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
    //
    IGL, // Illegal
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            //
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            //
            5 => return Opcode::HLT,
            //
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            //
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,
            //
            15 => return Opcode::JEQ,
            //
            _ => return Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
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
