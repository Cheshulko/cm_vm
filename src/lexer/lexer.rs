use nom::{do_parse, many1, named, types::CompleteStr};

use super::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

named!(pub program<CompleteStr, Program>, do_parse!(
    instructions: many1!(instruction_one) >> (Program {
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
}
