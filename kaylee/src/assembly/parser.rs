use std::collections::HashMap;
use std::iter::Map;

use maplit::hashmap;
use nom::{Err, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1, take_until, take_while, take_while1};
use nom::character::{is_alphabetic, is_newline};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, newline, space0, space1};
use nom::combinator::opt;
use nom::error::{ErrorKind, VerboseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

use crate::program::Program;

// @todo: This would require rewriting the instruction code itself to move the operand-MAP from the definition to the use

// @todo: Also, allow for
//@todo: - Labels
//@todo: - Subroutines
//@todo: - Comments


/// @todo: This is awful. Absolutely no error checking
pub fn into_bytecode(parsed: Vec<Vec<&str>>) -> Program {

// @todo: This is temporary. Find a way to build this dynamically
// @todo: https://users.rust-lang.org/t/macro-to-collect-metadata/75502
    let map: HashMap<&str, [u8; 4]> = hashmap! {
        "LOAD" => [30, 1, 2, 0],
        "ADD" => [70, 1, 1, 1],
    };

    // @todo: End temporary work

    let mut bytes: Vec<u8> = Vec::new();

    for instruction in parsed {
        // Get the info from the hashmap
        let item = map.get(instruction[0]).unwrap();

        // Push the opcode
        bytes.push(item[0].clone());

        for i in 1..instruction.len() {
            if let Some(value) = instruction.get(i) {
                // this is an operand, so we have to break it into u8 chunks
                let number = value.parse::<i32>().unwrap();
                let operand_bytes = number.to_be_bytes();

                let byte_count = item[i];
                let start_slice = (4 - byte_count) as usize;

                bytes.extend(&operand_bytes[start_slice..]);
            }
        }
    }

    Program::from(bytes)
}

pub fn parse_asm(s: &str) -> IResult<&str, Vec<Vec<&str>>, (&str, ErrorKind)> {
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

    use crate::assembly::parser::{instruction_parser, into_bytecode, is_valid_keyword_character, operand_parser, operation_keyword, parse_asm};
    use crate::program::Program;

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

    #[test]
    pub fn test_into_bytecode() {
        let parsed = vec![
            vec!["LOAD", "1", "500"],
            vec!["ADD", "2", "3", "2"],
        ];

        let expected = Program::from(vec![
            30, 1, 1, 244,
            70, 2, 3, 2,
        ]);

        let actual = into_bytecode(parsed);

        assert_eq!(expected, actual);
    }
}
