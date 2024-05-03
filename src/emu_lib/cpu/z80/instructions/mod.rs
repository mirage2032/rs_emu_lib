#![allow(non_camel_case_types)]

pub mod ld;
pub mod math;
pub mod nop;
pub mod halt;
pub mod rlca;
pub mod ex;
pub mod rrca;

#[macro_export]
macro_rules! generate_instruction_test {
    ($instruction:ident, [$($arg:expr),*]) => {
            use crate::cpu::z80::parser::Z80Parser;
            use crate::memory::memdevices::ROM;

            use super::*;

            paste::item! {
                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $instruction _as_bytes_and_back >]() {
                    let instruction = $instruction::new_with_value($($arg),*);
                    let ins_as_bytes = instruction.to_bytes();
                    let ins_as_rom: ROM = ins_as_bytes.clone().into();
                    let new_instruction = Z80Parser::from_memdev(&ins_as_rom, 0).expect(&format!("Failed to parse instruction: {:?}", ins_as_bytes));
                    assert_eq!(ins_as_bytes, new_instruction.to_bytes());
                }

                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $instruction _as_string_and_back >]() {
                    let instruction = $instruction::new_with_value($($arg),*);
                    let ins_as_string = instruction.to_string();
                    let new_instruction = Z80Parser::from_string(&ins_as_string).expect(&format!("Failed to parse instruction: {}", ins_as_string));
                    assert_eq!(ins_as_string, new_instruction.to_string());
                }
            }
    };
    ($instruction:ident) => {
            use crate::cpu::z80::parser::Z80Parser;
            use crate::memory::memdevices::ROM;

            use super::*;

            paste::item! {
                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $instruction _as_bytes_and_back >]() {
                    let instruction = $instruction::new();
                    let ins_as_bytes = instruction.to_bytes();
                    let ins_as_rom: ROM = ins_as_bytes.clone().into();
                    let new_instruction = Z80Parser::from_memdev(&ins_as_rom, 0).unwrap();
                    assert_eq!(ins_as_bytes, new_instruction.to_bytes());
                }

                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $instruction _as_string_and_back >]() {
                    let instruction = $instruction::new();
                    let ins_as_string = instruction.to_string();
                    let new_instruction = Z80Parser::from_string(&ins_as_string).unwrap();
                    assert_eq!(ins_as_string, new_instruction.to_string());
                }
            }
    };
}