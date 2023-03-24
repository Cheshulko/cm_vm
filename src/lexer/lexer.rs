use nom::{do_parse, many1, named, types::CompleteStr};

use super::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
// TODO: Dont like it. Rename
pub struct Lexer {
    instructions: Vec<AssemblerInstruction>,
}

impl Lexer {
    // TODO
    pub fn to_hex(&self) -> Vec<&str> {
        todo!()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }

    pub fn parse_instruction(input_instruction: &str) -> Result<AssemblerInstruction, ()> {
        let parsed = instruction_one(CompleteStr(input_instruction));

        match parsed {
            Ok(instruction_result) => {
                if instruction_result.0.is_empty() {
                    return Ok(instruction_result.1);
                } else {
                    return Err(());
                }
            }
            Result::Err(_) => Err(()),
        }
    }

    // TODO
    pub fn parse_program(input_program: &Vec<&str>) -> Lexer {
        todo!();
    }
}

// Root of parsing. Private
// TODO: Not finished yes
named!(program<CompleteStr, Lexer>, do_parse!(
    instructions: many1!(instruction_one) >> (Lexer {
        instructions: instructions
    })
));

mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
