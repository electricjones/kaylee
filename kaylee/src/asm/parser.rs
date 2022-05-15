use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, multispace0, newline, space0, space1};
use nom::character::is_alphabetic;
use nom::error::ErrorKind;
use nom::IResult;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, preceded};

use crate::asm::Parsed;

/// Parse any source string into a Parsed vector of strings
/// Does not actually parse to token enumerations. It simply splits a source into substrings.
/// The assembler takes these split strings and assembles them into true bytecode
pub fn parse_asm(s: &str) -> IResult<&str, Parsed, (&str, ErrorKind)> {
    separated_list1(many0(newline), line)(s)
}

/// Parse a single line into a vector of tokens
fn line(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    delimited(multispace0, instruction_parser, space0)(s)
}

/// Parse a single instruction into an operation and operands
fn instruction_parser(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    separated_list1(space1, alt((operation_keyword, operand_parser)))(s)
}

/// Parse a single keyword into a keyword token
fn operation_keyword(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(space0, take_while1(is_valid_keyword_character))(s)
}

/// Determine if a keyword is a valid operation
fn is_valid_keyword_character(c: char) -> bool {
    is_alphabetic(c as u8) || c == '.' || c == '_'
}

/// Parse an operand
fn operand_parser(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(alt((tag("$"), tag("#"))), digit1)(s)
}


#[cfg(test)]
mod test {
    use nom::Err::Error;
    use nom::error::ErrorKind;

    use crate::asm::parser::{instruction_parser, is_valid_keyword_character, operand_parser, operation_keyword, parse_asm};

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
    pub fn test_parse_full_messy_example() {
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

        assert_eq!(expected, parse_asm(input).unwrap().1);
    }
}
