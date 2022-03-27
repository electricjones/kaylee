use std::num::ParseIntError;

pub fn parse_hex(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let split = hex.split(" ").collect::<Vec<&str>>();
    let mut results: Vec<u8> = vec![];
    for hex_string in split {
        let byte = u8::from_str_radix(&hex_string, 16);
        match byte {
            Ok(result) => { results.push(result) }
            Err(e) => { return Err(e) }
        }
    }

    Ok(results)
}