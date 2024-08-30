use serde::{Deserialize, Serialize};

use crate::cpu::CPUType;
use crate::cpu::registers::BaseRegister;
use crate::emulator::Emulator;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::memdevices::RAM;

macro_rules! test_instruction_parse {
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
pub(crate) use test_instruction_parse;

