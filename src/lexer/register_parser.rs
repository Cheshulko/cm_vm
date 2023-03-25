use nom::{digit, named, tag, types::CompleteStr, ws};

use super::token::Token;

named!(pub register<CompleteStr, Token>, ws!(
    do_parse!(
        tag!("$") >> reg_num: digit >> (Token::Register {
            reg_num: reg_num.parse::<u8>().unwrap()
        })
    )
));

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        // let r = result.ok().unwrap();
        // println!("{:?} {:?}", r.0, r.1);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}
