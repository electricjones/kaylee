use nom::{Err, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1, take_until, take_while, take_while1};
use nom::character::{is_alphabetic, is_newline};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, newline, space0, space1};
use nom::combinator::opt;
use nom::error::{ErrorKind, VerboseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

// @todo: This would require rewriting the instruction code itself to move the operand-map from the definition to the use

// @todo: Also, allow for
//@todo: - Labels
//@todo: - Subroutines
//@todo: - Comments

fn parse_asm(s: &str) -> IResult<&str, Vec<Vec<&str>>, (&str, ErrorKind)> {
    separated_list1(many0(newline), line)(s)
}

fn line(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    delimited(multispace0, instruction_parser, space0)(s)
}

fn instruction_parser(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    separated_list1(space1, alt((operation_keyword, operand_parser)))(s)
}

fn operation_keyword(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(space0, take_while1(is_valid_keyword_character))(s)
}

fn is_valid_keyword_character(c: char) -> bool {
    is_alphabetic(c as u8) || c == '.' || c == '_'
}

fn operand_parser(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(alt((tag("$"), tag("#"))), digit1)(s)
}


#[cfg(test)]
mod test {
    use nom::Err::Error;
    use nom::error::ErrorKind;
    use nom::IResult;

    use crate::assembly::parser::{instruction_parser, is_valid_keyword_character, operand_parser, operation_keyword, parse_asm};

    #[test]
    pub fn test_is_valid_keyword_character() {
        assert!(is_valid_keyword_character('A'));
        assert!(is_valid_keyword_character('a'));
        assert!(is_valid_keyword_character('.'));
        assert!(is_valid_keyword_character('_'));

        assert_eq!(false, is_valid_keyword_character('&'));
        assert_eq!(false, is_valid_keyword_character('$'));
        assert_eq!(false, is_valid_keyword_character('`'));
        assert_eq!(false, is_valid_keyword_character('-'));
    }

    #[test]
    pub fn test_operation_keyword_parser() {
        assert_eq!("LOAD", operation_keyword("LOAD").unwrap().1);
        assert_eq!("LOAD", operation_keyword(" LOAD").unwrap().1);
        assert_eq!("LOAD", operation_keyword("LOAD ").unwrap().1);
        assert_eq!("LOAD", operation_keyword(" LOAD ").unwrap().1);
        assert_eq!("LOAD", operation_keyword("  LOAD ").unwrap().1);
        assert_eq!("LOAD", operation_keyword("  LOAD  ").unwrap().1);
        assert_eq!("LOAD", operation_keyword("\tLOAD        ").unwrap().1);
    }

    #[test]
    pub fn test_operand_parser() {
        assert_eq!(("", "1"), operand_parser("$1").unwrap());
        assert_eq!(("", "233"), operand_parser("#233").unwrap());

        assert_eq!(
            operand_parser("^1"),
            Err(Error(("^1", ErrorKind::Tag)))
        );
    }

    #[test]
    pub fn test_instruction() {
        assert_eq!(vec!["LOAD", "0", "500"], instruction_parser("LOAD $0 #500").unwrap().1);
        assert_eq!(vec!["LOAD", "3", "18"], instruction_parser("LOAD #3 $18").unwrap().1);
    }

    #[test]
    pub fn test_full_example() {
        let input = r#"

                 LOAD    $1   #500
 ADD $2     $3 $2


DIE #1                      


    HALT    


"#;

        let expected = vec![
            vec!["LOAD", "1", "500"],
            vec!["ADD", "2", "3", "2"],
            vec!["DIE", "1"],
            vec!["HALT"],
        ];
    }
}
