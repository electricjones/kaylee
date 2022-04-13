use quote::quote;

enum SignatureState {
    Identifier,
    Operands,
}

fn main() {
    let mut identifier = String::new();
    let mut operands = [quote! {}, quote! {}, quote! {}];

    let mut buffer = String::new();
    let mut state = SignatureState::Identifier;
    let mut operand_index = 0;

    for char in "LOAD $D #2 ".chars() {
        if char == ' ' {
            state = match state {
                SignatureState::Identifier => {
                    identifier = String::from(buffer.clone());
                    SignatureState::Operands
                }
                SignatureState::Operands => {
                    let me = buffer.chars().nth(0).unwrap();
                    let operand_tokens = match me {
                        '$' => {
                            quote! { OperandType::RegisterId }
                        }
                        '#' => {
                            let bytes = &buffer[1..].parse::<i32>().unwrap();
                            match bytes {
                                1 => quote! { OperandType::ConstantByte },
                                2 => quote! { OperandType::ConstantHalfWord },
                                3 => quote! { OperandType::ConstantWord },
                                _ => panic!("Constant Value too Large")
                            }
                        }
                        _ => {
                            panic!("Invalid Signature: {}", me)
                        }
                    };

                    operand_index = operand_index + 1;
                    operands[operand_index] = operand_tokens;

                    SignatureState::Operands
                }
            };

            buffer = String::new();
            continue;
        }

        buffer.push(char);
    }
}