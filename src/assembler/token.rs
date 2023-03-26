use crate::vm::instruction::Opcode;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    Number { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
