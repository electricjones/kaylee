use nom::{Err, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1, take_until, take_while, take_while1};
use nom::character::{is_alphabetic, is_newline};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, newline, space0, space1};
use nom::combinator::opt;
use nom::error::{ErrorKind, VerboseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

fn parser(s: &str) -> IResult<&str, Vec<Vec<&str>>, (&str, ErrorKind)> {
    separated_list1(many0(newline), line)(s)
}

fn line(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    delimited(multispace0, instruction, space0)(s)
}

fn instruction(s: &str) -> IResult<&str, Vec<&str>, (&str, ErrorKind)> {
    separated_list1(space1, alt((op_keyword, operand)))(s)
}

fn op_keyword(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(space0, take_while1(is_valid_op_keyword))(s)
}

fn is_valid_op_keyword(c: char) -> bool {
    is_alphabetic(c as u8) || c == '.' || c == '_'
}

fn operand(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    preceded(alt((tag("$"), tag("#"))), digit1)(s)
}


#[cfg(test)]
mod test {
    use crate::assembly::parser::parser;

    #[test]
    pub fn test_sandbox_parsing() {
        let input = r#"

                 LOAD    $1   #500
 ADD $2     $3 $2


DIE #1                      


    HALT    


"#;

        let result = parser(input);
        match result {
            Ok((_, good)) => {
                println!("good");
            }
            Err(bad) => {
                println!("bad")
            }
        }
    }
}
