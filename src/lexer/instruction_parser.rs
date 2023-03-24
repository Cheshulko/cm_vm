use nom::{do_parse, named, opt, types::CompleteStr};

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
