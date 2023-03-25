use log::error;
use nom::{alt, multispace};
use nom::{do_parse, named, opt, types::CompleteStr};

use crate::vm;

use super::opcode_parser::*;
use super::operand_parser::*;
use super::register_parser::*;
use super::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_hex(&self) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        match self.opcode {
            Token::Op { code } => {
                // TODO: Fix this conversion
                let bytes = code as u8;
                let hex = format!("{:02x?}", bytes);
                results.push(hex);
            }
            _ => {
                error!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                self.extract_hex_operand(token, &mut results);
            }
        }

        results
    }

    // From AssemblerInstruction to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => {
                // TODO: Fix this conversion
                results.push(code as u8);
            }
            _ => {
                error!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                self.extract_operand(token, &mut results)
            }
        }

        results
    }

    // Token -> hex
    fn extract_hex_operand(&self, token: &Token, hex_results: &mut Vec<String>) {
        match token {
            Token::Register { reg_num } => {
                let hex = format!("{:02x?}", *reg_num);
                hex_results.push(hex);
            }
            Token::Number { value } => {
                let converted = *value as u16;
                let hex1 = format!("{:02x?}", converted as u8);
                let hex2 = format!("{:02x?}", (converted >> vm::VM::SIZE) as u8);

                hex_results.push(hex2);
                hex_results.push(hex1);
            }
            _ => {
                error!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }

    // Token -> u8
    fn extract_operand(&self, token: &Token, results: &mut Vec<u8>) {
        match token {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::Number { value } => {
                let converted = *value as u16;
                let byte1 = converted as u8;
                // Invert to vm::VM::next_16_bits
                let byte2 = (converted >> vm::VM::SIZE) as u8; // Take 8 higher
                results.push(byte2);
                results.push(byte1); // Note: Convert here u16->u18 just throws 8 higher bits and take lower
            }
            _ => {
                error!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: opt!(register) >>
        r2: opt!(alt!(register | integer_operand)) >>
        r3: opt!(alt!(register | integer_operand)) >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: r1,
                operand2: r2,
                operand3: r3
            }
        )
    )
);

mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_load() {
        let result = instruction(CompleteStr("load $0 #100\n"));
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

    #[test]
    fn test_parse_instruction_add() {
        let result = instruction(CompleteStr("add $0 $5 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 5 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_jumpf() {
        let result = instruction(CompleteStr("jmpf $1\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::JMPF },
                    operand1: Some(Token::Register { reg_num: 1 }),
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_eq() {
        let result = instruction(CompleteStr("eq $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::EQ },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 })
                }
            ))
        );
    }

    // TODO: Add tests for other commands
    // NEQ -- LTQ

    #[test]
    fn test_parse_instruction_hlt() {
        let result = instruction(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
