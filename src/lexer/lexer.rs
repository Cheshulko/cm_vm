use nom::{do_parse, many1, named, types::CompleteStr};

use super::instruction_parser::{instruction, AssemblerInstruction};

#[derive(Debug, PartialEq, Eq)]
// TODO: Dont like it. Rename
pub struct Lexer {
    instructions: Vec<AssemblerInstruction>,
}

impl Lexer {
    // TODO
    #[allow(dead_code)]
    pub fn to_hex(&self) -> Vec<&str> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }

    pub fn parse_instruction(input_instruction: &str) -> Result<AssemblerInstruction, ()> {
        let parsed = instruction(CompleteStr(input_instruction));

        match parsed {
            Ok(instruction_result) => {
                println!("[LX] Parsed {:?}", instruction_result.1);
                if instruction_result.0.is_empty() {
                    Ok(instruction_result.1)
                } else {
                    Err(())
                }
            }
            Result::Err(_) => Err(()),
        }
    }

    // TODO
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn parse_program(input_program: &[&str]) -> Lexer {
        todo!();
    }
}

// Root of parsing. Private
// TODO: Not finished yet
named!(program<CompleteStr, Lexer>, do_parse!(
    instructions: many1!(instruction) >> (Lexer {
        instructions
    })
));

mod tests {
    use super::*;

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
