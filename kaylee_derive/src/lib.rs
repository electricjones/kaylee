extern crate proc_macro;

use proc_macro::TokenStream;

// use proc_macro2::Ident;
use quote::quote;
use syn::{DeriveInput, Lit, Meta, parse_macro_input};
use syn::Lit::Str;
// use syn::Lit::Str;
use syn::parse::Parser;

// use std::process::id;

// use syn::parse_macro_input::ParseMacroInput;

enum SignatureState {
    Identifier,
    Operands,
}

#[proc_macro_derive(Instruction, attributes(opcode, signature))]
pub fn derive_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let mut opcode: u8 = 0;
    let mut help = String::new();
    let mut identifier = String::new();
    let mut operands = [
        quote! { OperandType::None },
        quote! { OperandType::None },
        quote! { OperandType::None },
    ];

    let attributes = input.attrs;
    for attribute in attributes {
        let meta = attribute.parse_meta().unwrap();
        if let Meta::NameValue(value) = meta {
            if let Some(ident) = value.path.get_ident() {
                match ident.to_string().as_str() {
                    "opcode" => {
                        if let Lit::Int(lit) = value.lit {
                            opcode = lit.base10_parse::<u8>().unwrap();
                        }
                    }
                    "doc" => {
                        if let Lit::Str(lit) = value.lit {
                            help.push('\n');
                            help += &lit.value();
                        }
                    }
                    "signature" => {
                        if let Lit::Str(lit) = value.lit {
                            let mut buffer = String::new();
                            let mut state = SignatureState::Identifier;
                            let mut operand_index = 0;

                            let mut me = lit.value();
                            me.push(' ');

                            let sig_lit = me.chars();
                            for char in sig_lit {
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

                                            operands[operand_index] = operand_tokens;
                                            operand_index = operand_index + 1;

                                            SignatureState::Operands
                                        }
                                    };

                                    buffer = String::new();
                                    continue;
                                }

                                buffer.push(char);
                            }
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    let name = format!("{}", struct_name);
    let op1 = &operands[0];
    let op2 = &operands[1];
    let op3 = &operands[2];

    let tokens = quote! {
        impl #struct_name {
            pub const OPCODE: u8 = #opcode;
        }

        impl Instruction for #struct_name {
            fn new(operand_values: OperandValues) -> Self {
                #struct_name { operand_values }
            }

            fn signature() -> InstructionSignature where Self: Sized {
                InstructionSignature {
                    identifier: String::from(#identifier),
                    operands: [#op1, #op2, #op3]
                }
            }

            fn documentation() -> InstructionDocumentation where Self: Sized {
                InstructionDocumentation {
                    name: String::from(#name),
                    help: String::from(#help),
                }
            }

            fn display(&self) -> String {
                display_instruction_with_values(self)
            }

            fn operand_values(&self) -> &OperandValues {
                &self.operand_values
            }
        }
    };

    tokens.into()
}

// @todo: I don't think this is needed
#[proc_macro_attribute]
pub fn values(_args: TokenStream, input: TokenStream) -> TokenStream {
    // We need to add the operand_values field.
    // @todo: This is probably not the best place for this, but it has to be in an attribute, not the derive()
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! {
                            operand_values: OperandValues
                        }).unwrap());
                }
                _ => {
                    ()
                }
            }

            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}
