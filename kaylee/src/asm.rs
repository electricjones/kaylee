use nom::error::ErrorKind;
use nom::IResult;

pub mod parser;
pub mod assembler;

pub struct Source {
    pub body: String,
}

impl Source {
    pub fn from(body: String) -> Self {
        Self {
            body
        }
    }
}

pub type Parsed<'a> = Vec<Vec<&'a str>>;
// 
// impl<'a> TryFrom<Source> for Parsed<'a> {
//     type Error = ErrorKind;
// 
//     fn try_from(source: Source) -> Result<Self, Self::Error> {
//         let parsed = parser::parse_asm(source.body.as_str());
//         match parsed { 
//             Ok(success) => Ok(success.1),
//             Err(nom::Err::Error(failure)) => Err(failure.1),
//             _ => Err(ErrorKind::Fail)
//         }
//     }
// }
