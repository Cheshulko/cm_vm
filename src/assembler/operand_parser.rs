use nom::{digit, named, tag, types::CompleteStr, ws};

use super::token::Token;

named!(pub integer_operand<CompleteStr, Token>, ws!(
    do_parse!(
        tag!("#") >> reg_num: digit >> (Token::Number {
            value: reg_num.parse::<i32>().unwrap()
        })
    )
));

mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        // let r = result.ok().unwrap();
        // println!("{:?} {:?}", r.0, r.1);
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Number { value: 10 });

        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
}
