use nom::{alpha1, alt, do_parse, named, opt, tag, types::CompleteStr, ws};

use crate::assembler::register_parser::register;

use super::{
    instruction_parser::AssemblerInstruction, operand_parser::integer_operand, token::Token,
};

named!(directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >> name: alpha1 >> (Token::Directive {
            name: name.to_string()
        })
));

named!(directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            name: directive_declaration >>
            o1: opt!(alt!(register | integer_operand)) >>
            o2: opt!(alt!(register | integer_operand)) >>
            o3: opt!(alt!(register | integer_operand)) >>
            (
                AssemblerInstruction{
                    opcode: None,
                    directive: Some(name),
                    label: None,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

/*
<directive>
<directive> <register | integer_operand>
<directive> <register | integer_operand> <register | integer_operand>
<directive> <register | integer_operand> <register | integer_operand> <register | integer_operand>
*/
named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(directive_combined) >>
        (
            ins
        )
    )
);

mod tests {
    use super::*;
    use crate::vm::instruction::Opcode;

    #[test]
    fn test_parse_directive_declaration() {
        let result = directive_declaration(CompleteStr(".directive"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                Token::Directive {
                    name: "directive".to_string()
                }
            )),
        );
    }

    #[test]
    fn test_parse_directive() {
        let result = directive(CompleteStr(".directive"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_register() {
        let result = directive(CompleteStr(".directive $0"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_integer_operand() {
        let result = directive(CompleteStr(".directive #10"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Number { value: 10 }),
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_register_register() {
        let result = directive(CompleteStr(".directive $0 $1"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_integer_operand_integer_operand() {
        let result = directive(CompleteStr(".directive #10 #20"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Number { value: 10 }),
                    operand2: Some(Token::Number { value: 20 }),
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_register_register_integer_operand() {
        let result = directive(CompleteStr(".directive $0 $1 #100"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Number { value: 100 }),
                }
            ))
        );
    }

    #[test]
    fn test_parse_directive_integer_operand_integer_operand_register() {
        let result = directive(CompleteStr(".directive #10 #20 $5"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        name: "directive".to_string()
                    }),
                    label: None,
                    operand1: Some(Token::Number { value: 10 }),
                    operand2: Some(Token::Number { value: 20 }),
                    operand3: Some(Token::Register { reg_num: 5 }),
                }
            ))
        );
    }
}
