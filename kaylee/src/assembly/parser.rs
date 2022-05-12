use linkme::distributed_slice;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::is_alphabetic;
use nom::character::complete::{digit1, multispace0, newline, space0, space1};
use nom::error::ErrorKind;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, preceded};

use crate::instructions::OperandType;
use crate::program::Program;

// @todo: This would require rewriting the instruction code itself to move the operand-MAP from the definition to the use

// @todo: Also, allow for
//@todo: - Labels
//@todo: - Subroutines
//@todo: - Comments

pub type MapOperands = (&'static str, u8, [OperandType; 3]);

#[distributed_slice]
pub static MY_MAP: [MapOperands] = [..];

// #[distributed_slice(MY_MAP)]
// static N: i32 = 9;
// static N: (&'static str, u8, u8, u8, u8) = ("LOAD", 30, 1, 2, 0);

// #[distributed_slice(SHENANIGANS)]
// static NNN: i32 = 999;

/// @todo: This is awful. Absolutely no error checking
pub fn into_bytecode(parsed: Vec<Vec<&str>>) -> Program {

// @todo: This is temporary. Find a way to build this dynamically
// @todo: https://users.rust-lang.org/t/macro-to-collect-metadata/75502
//     let map: HashMap<&str, [u8; 4]> = hashmap! {
//         "LOAD" => [30, 1, 2, 0],
//         "ADD" => [70, 1, 1, 1],
//     };
//     
//     for _i in MY_MAP {
//         let a = true;
//     }

    // @todo: End temporary work

    let mut bytes: Vec<u8> = Vec::new();

    for instruction in parsed {
        // Get the info from the hashmap
        let mut item: Option<&MapOperands> = None;
        for a in MY_MAP {
            if a.0 == instruction[0] {
                item = Some(a);
                break;
            }
        }

        // Push the opcode
        bytes.push(item.unwrap().1.clone());

        for i in 1..(instruction.len()) {
            if let Some(value) = instruction.get(i) {
                // this is an operand, so we have to break it into u8 chunks
                let number = value.parse::<i32>().unwrap();
                let operand_bytes = number.to_be_bytes();

                let spot: &OperandType = &item.unwrap().2[i - 1];

                let byte_count = match spot {
                    OperandType::None => 0 as u8,
                    OperandType::RegisterId => 1 as u8,
                    OperandType::ConstantByte => 1 as u8,
                    OperandType::ConstantHalfWord => 2 as u8,
                    OperandType::ConstantWord => 3 as u8,
                };

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

        assert_eq!(expected, parse_asm(input).unwrap().1);
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
