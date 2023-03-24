use nom::{do_parse, named, opt, types::CompleteStr};

use crate::vm;

use super::opcode_parser::*;
use super::operand_parser::*;
use super::register_parser::*;
use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    // TODO
    pub fn to_hex(&self) -> Vec<&str> {
        todo!()
    }
    // From AssemblerInstruction to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                // TODO: load, add, etc
                _ => {
                    // TODO: Fix this conversion
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(token) => self.extract_operand(token, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(&self, token: &Token, results: &mut Vec<u8>) {
        match token {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::Number { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                // Invert to vm::VM::next_16_bits
                let byte2 = converted >> vm::VM::SIZE; // Take 8 higher
                results.push(byte2 as u8);
                results.push(byte1 as u8); // Note: Conver here u16->u18 just throws 8 higher bits and take lower
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

// LOAD $0 #100
named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
        r: opt!(register) >>
        i: opt!(integer_operand) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: r,
                operand2: i,
                operand3: None
            }
        )
    )
);

mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Number { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
