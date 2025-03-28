use regex::Regex;

use crate::cpu::instruction::{ExecutableInstruction, InstructionParser, ParseError};
use crate::cpu::z80::instructions::*;
use crate::cpu::z80::Z80;
use crate::memory::MemoryDevice;

#[derive(Debug, Clone)]
enum ImmediateValue {
    Val8(u8),
    Val16(u16),
    OffsetIX(u8),
    OffsetIY(u8),
    Ptr(u16),
}

fn is_num(number: &str) -> Result<u16, String> {
    let num = if number.starts_with("0x") && number.len() <= 6 {
        u16::from_str_radix(&number[2..], 16).map_err(|e| e.to_string())?
    } else if number.starts_with("0b") && number.len() <= 18 {
        u16::from_str_radix(&number[2..], 2).map_err(|e| e.to_string())?
    // } else if number.len() <= 3 {
    //     Number::U8(u8::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    // } else if number.len() <= 5 {
    //     Number::U16(u16::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    } else {
        u16::from_str_radix(&number, 10).map_err(|e| e.to_string())?
    };
    Ok(num)
}

fn is_val(number: &str) -> Result<ImmediateValue, String> {
    if number.starts_with("(") && number.ends_with(")") {
        let parsed = &number[1..number.len() - 1];
        if parsed.contains("+") {
            match &parsed[0..2] {
                "ix" => {
                    let offset = parsed.split("+").collect::<Vec<&str>>()[1];
                    let num = is_num(offset)?;
                    return Ok(ImmediateValue::OffsetIX(num as u8));
                }
                "iy" => {
                    let offset = parsed.split("+").collect::<Vec<&str>>()[1];
                    let num = is_num(offset)?;
                    return Ok(ImmediateValue::OffsetIY(num as u8));
                }
                _ => (),
            };
        }
        Ok(ImmediateValue::Ptr(is_num(parsed)?))
    } else {
        let num = is_num(number)?;
        if number.len() != 6 {
            Ok(ImmediateValue::Val8(num as u8))
        } else {
            Ok(ImmediateValue::Val16(num))
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Z80Parser {}

impl InstructionParser<Z80> for Z80Parser {
    fn ins_from_asm_string(
        &self,
        instruction: &str,
    ) -> Result<Box<(dyn ExecutableInstruction<Z80>)>, ParseError> {
        let filtered = instruction.to_lowercase().replace(",", " ");
        //regex
        let re = Regex::new(r"^([a-z]+)(?: +([(a-z0-9+')]+)(?: ?+,? ?+([(a-z0-9+')]+))?)?$")
            .expect("Error building Z80 instruction parsing regex");
        let op = match re.captures(&filtered) {
            Some(caps) => caps,
            None => {
                return Err(ParseError::InvalidInstruction(format!(
                    "Invalid instruction: {}",
                    instruction
                )))
            }
        };
        let get_op = |i: usize| -> Result<&str, ParseError> {
            op.get(i)
                .ok_or(ParseError::InvalidInstruction(format!(
                    "Invalid instruction: {}",
                    instruction
                )))
                .map(|m| m.as_str())
        };
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match get_op(1)? {
            "nop" => Box::new(nop::NOP::new()),
            "scf" => Box::new(scf::SCF::new()),
            "ccf" => Box::new(ccf::CCF::new()),
            "exx" => Box::new(exx::EXX::new()),
            "retn" => Box::new(retn::RETN::new()),
            "di" => Box::new(di::DI::new()),
            "ei" => Box::new(ei::EI::new()),
            "neg" => Box::new(neg::NEG::new()),
            "bit" => {
                let bit = get_op(2)?;
                let destination = get_op(3)?;
                match (is_val(bit), destination, is_val(destination)) {
                    (Ok(ImmediateValue::Val8(0)), "b", _) => Box::new(bit::bit::BIT_0_B::new()),
                    (Ok(ImmediateValue::Val8(0)), "c", _) => Box::new(bit::bit::BIT_0_C::new()),
                    (Ok(ImmediateValue::Val8(0)), "d", _) => Box::new(bit::bit::BIT_0_D::new()),
                    (Ok(ImmediateValue::Val8(0)), "e", _) => Box::new(bit::bit::BIT_0_E::new()),
                    (Ok(ImmediateValue::Val8(0)), "h", _) => Box::new(bit::bit::BIT_0_H::new()),
                    (Ok(ImmediateValue::Val8(0)), "l", _) => Box::new(bit::bit::BIT_0_L::new()),
                    (Ok(ImmediateValue::Val8(0)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_0_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(0)), "a", _) => Box::new(bit::bit::BIT_0_A::new()),
                    (Ok(ImmediateValue::Val8(1)), "b", _) => Box::new(bit::bit::BIT_1_B::new()),
                    (Ok(ImmediateValue::Val8(1)), "c", _) => Box::new(bit::bit::BIT_1_C::new()),
                    (Ok(ImmediateValue::Val8(1)), "d", _) => Box::new(bit::bit::BIT_1_D::new()),
                    (Ok(ImmediateValue::Val8(1)), "e", _) => Box::new(bit::bit::BIT_1_E::new()),
                    (Ok(ImmediateValue::Val8(1)), "h", _) => Box::new(bit::bit::BIT_1_H::new()),
                    (Ok(ImmediateValue::Val8(1)), "l", _) => Box::new(bit::bit::BIT_1_L::new()),
                    (Ok(ImmediateValue::Val8(1)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_1_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(1)), "a", _) => Box::new(bit::bit::BIT_1_A::new()),
                    (Ok(ImmediateValue::Val8(2)), "b", _) => Box::new(bit::bit::BIT_2_B::new()),
                    (Ok(ImmediateValue::Val8(2)), "c", _) => Box::new(bit::bit::BIT_2_C::new()),
                    (Ok(ImmediateValue::Val8(2)), "d", _) => Box::new(bit::bit::BIT_2_D::new()),
                    (Ok(ImmediateValue::Val8(2)), "e", _) => Box::new(bit::bit::BIT_2_E::new()),
                    (Ok(ImmediateValue::Val8(2)), "h", _) => Box::new(bit::bit::BIT_2_H::new()),
                    (Ok(ImmediateValue::Val8(2)), "l", _) => Box::new(bit::bit::BIT_2_L::new()),
                    (Ok(ImmediateValue::Val8(2)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_2_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(2)), "a", _) => Box::new(bit::bit::BIT_2_A::new()),
                    (Ok(ImmediateValue::Val8(3)), "b", _) => Box::new(bit::bit::BIT_3_B::new()),
                    (Ok(ImmediateValue::Val8(3)), "c", _) => Box::new(bit::bit::BIT_3_C::new()),
                    (Ok(ImmediateValue::Val8(3)), "d", _) => Box::new(bit::bit::BIT_3_D::new()),
                    (Ok(ImmediateValue::Val8(3)), "e", _) => Box::new(bit::bit::BIT_3_E::new()),
                    (Ok(ImmediateValue::Val8(3)), "h", _) => Box::new(bit::bit::BIT_3_H::new()),
                    (Ok(ImmediateValue::Val8(3)), "l", _) => Box::new(bit::bit::BIT_3_L::new()),
                    (Ok(ImmediateValue::Val8(3)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_3_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(3)), "a", _) => Box::new(bit::bit::BIT_3_A::new()),
                    (Ok(ImmediateValue::Val8(4)), "b", _) => Box::new(bit::bit::BIT_4_B::new()),
                    (Ok(ImmediateValue::Val8(4)), "c", _) => Box::new(bit::bit::BIT_4_C::new()),
                    (Ok(ImmediateValue::Val8(4)), "d", _) => Box::new(bit::bit::BIT_4_D::new()),
                    (Ok(ImmediateValue::Val8(4)), "e", _) => Box::new(bit::bit::BIT_4_E::new()),
                    (Ok(ImmediateValue::Val8(4)), "h", _) => Box::new(bit::bit::BIT_4_H::new()),
                    (Ok(ImmediateValue::Val8(4)), "l", _) => Box::new(bit::bit::BIT_4_L::new()),
                    (Ok(ImmediateValue::Val8(4)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_4_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(4)), "a", _) => Box::new(bit::bit::BIT_4_A::new()),
                    (Ok(ImmediateValue::Val8(5)), "b", _) => Box::new(bit::bit::BIT_5_B::new()),
                    (Ok(ImmediateValue::Val8(5)), "c", _) => Box::new(bit::bit::BIT_5_C::new()),
                    (Ok(ImmediateValue::Val8(5)), "d", _) => Box::new(bit::bit::BIT_5_D::new()),
                    (Ok(ImmediateValue::Val8(5)), "e", _) => Box::new(bit::bit::BIT_5_E::new()),
                    (Ok(ImmediateValue::Val8(5)), "h", _) => Box::new(bit::bit::BIT_5_H::new()),
                    (Ok(ImmediateValue::Val8(5)), "l", _) => Box::new(bit::bit::BIT_5_L::new()),
                    (Ok(ImmediateValue::Val8(5)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_5_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(5)), "a", _) => Box::new(bit::bit::BIT_5_A::new()),
                    (Ok(ImmediateValue::Val8(6)), "b", _) => Box::new(bit::bit::BIT_6_B::new()),
                    (Ok(ImmediateValue::Val8(6)), "c", _) => Box::new(bit::bit::BIT_6_C::new()),
                    (Ok(ImmediateValue::Val8(6)), "d", _) => Box::new(bit::bit::BIT_6_D::new()),
                    (Ok(ImmediateValue::Val8(6)), "e", _) => Box::new(bit::bit::BIT_6_E::new()),
                    (Ok(ImmediateValue::Val8(6)), "h", _) => Box::new(bit::bit::BIT_6_H::new()),
                    (Ok(ImmediateValue::Val8(6)), "l", _) => Box::new(bit::bit::BIT_6_L::new()),
                    (Ok(ImmediateValue::Val8(6)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_6_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(6)), "a", _) => Box::new(bit::bit::BIT_6_A::new()),
                    (Ok(ImmediateValue::Val8(7)), "b", _) => Box::new(bit::bit::BIT_7_B::new()),
                    (Ok(ImmediateValue::Val8(7)), "c", _) => Box::new(bit::bit::BIT_7_C::new()),
                    (Ok(ImmediateValue::Val8(7)), "d", _) => Box::new(bit::bit::BIT_7_D::new()),
                    (Ok(ImmediateValue::Val8(7)), "e", _) => Box::new(bit::bit::BIT_7_E::new()),
                    (Ok(ImmediateValue::Val8(7)), "h", _) => Box::new(bit::bit::BIT_7_H::new()),
                    (Ok(ImmediateValue::Val8(7)), "l", _) => Box::new(bit::bit::BIT_7_L::new()),
                    (Ok(ImmediateValue::Val8(7)), "(hl)", _) => {
                        Box::new(bit::bit::BIT_7_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(7)), "a", _) => Box::new(bit::bit::BIT_7_A::new()),
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_0_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_1_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_2_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_3_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_4_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_5_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_6_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::bit::BIT_7_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_0_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_1_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_2_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_3_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_4_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_5_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_6_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::bit::BIT_7_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid BIT operands \"{0}\" and \"{1}\"",
                            bit, destination
                        )))
                    }
                }
            }
            "res" => {
                let bit = get_op(2)?;
                let destination = get_op(3)?;
                match (is_val(bit), destination, is_val(destination)) {
                    (Ok(ImmediateValue::Val8(0)), "b", _) => Box::new(bit::res::RES_0_B::new()),
                    (Ok(ImmediateValue::Val8(0)), "c", _) => Box::new(bit::res::RES_0_C::new()),
                    (Ok(ImmediateValue::Val8(0)), "d", _) => Box::new(bit::res::RES_0_D::new()),
                    (Ok(ImmediateValue::Val8(0)), "e", _) => Box::new(bit::res::RES_0_E::new()),
                    (Ok(ImmediateValue::Val8(0)), "h", _) => Box::new(bit::res::RES_0_H::new()),
                    (Ok(ImmediateValue::Val8(0)), "l", _) => Box::new(bit::res::RES_0_L::new()),
                    (Ok(ImmediateValue::Val8(0)), "(hl)", _) => {
                        Box::new(bit::res::RES_0_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(0)), "a", _) => Box::new(bit::res::RES_0_A::new()),
                    (Ok(ImmediateValue::Val8(1)), "b", _) => Box::new(bit::res::RES_1_B::new()),
                    (Ok(ImmediateValue::Val8(1)), "c", _) => Box::new(bit::res::RES_1_C::new()),
                    (Ok(ImmediateValue::Val8(1)), "d", _) => Box::new(bit::res::RES_1_D::new()),
                    (Ok(ImmediateValue::Val8(1)), "e", _) => Box::new(bit::res::RES_1_E::new()),
                    (Ok(ImmediateValue::Val8(1)), "h", _) => Box::new(bit::res::RES_1_H::new()),
                    (Ok(ImmediateValue::Val8(1)), "l", _) => Box::new(bit::res::RES_1_L::new()),
                    (Ok(ImmediateValue::Val8(1)), "(hl)", _) => {
                        Box::new(bit::res::RES_1_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(1)), "a", _) => Box::new(bit::res::RES_1_A::new()),
                    (Ok(ImmediateValue::Val8(2)), "b", _) => Box::new(bit::res::RES_2_B::new()),
                    (Ok(ImmediateValue::Val8(2)), "c", _) => Box::new(bit::res::RES_2_C::new()),
                    (Ok(ImmediateValue::Val8(2)), "d", _) => Box::new(bit::res::RES_2_D::new()),
                    (Ok(ImmediateValue::Val8(2)), "e", _) => Box::new(bit::res::RES_2_E::new()),
                    (Ok(ImmediateValue::Val8(2)), "h", _) => Box::new(bit::res::RES_2_H::new()),
                    (Ok(ImmediateValue::Val8(2)), "l", _) => Box::new(bit::res::RES_2_L::new()),
                    (Ok(ImmediateValue::Val8(2)), "(hl)", _) => {
                        Box::new(bit::res::RES_2_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(2)), "a", _) => Box::new(bit::res::RES_2_A::new()),
                    (Ok(ImmediateValue::Val8(3)), "b", _) => Box::new(bit::res::RES_3_B::new()),
                    (Ok(ImmediateValue::Val8(3)), "c", _) => Box::new(bit::res::RES_3_C::new()),
                    (Ok(ImmediateValue::Val8(3)), "d", _) => Box::new(bit::res::RES_3_D::new()),
                    (Ok(ImmediateValue::Val8(3)), "e", _) => Box::new(bit::res::RES_3_E::new()),
                    (Ok(ImmediateValue::Val8(3)), "h", _) => Box::new(bit::res::RES_3_H::new()),
                    (Ok(ImmediateValue::Val8(3)), "l", _) => Box::new(bit::res::RES_3_L::new()),
                    (Ok(ImmediateValue::Val8(3)), "(hl)", _) => {
                        Box::new(bit::res::RES_3_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(3)), "a", _) => Box::new(bit::res::RES_3_A::new()),
                    (Ok(ImmediateValue::Val8(4)), "b", _) => Box::new(bit::res::RES_4_B::new()),
                    (Ok(ImmediateValue::Val8(4)), "c", _) => Box::new(bit::res::RES_4_C::new()),
                    (Ok(ImmediateValue::Val8(4)), "d", _) => Box::new(bit::res::RES_4_D::new()),
                    (Ok(ImmediateValue::Val8(4)), "e", _) => Box::new(bit::res::RES_4_E::new()),
                    (Ok(ImmediateValue::Val8(4)), "h", _) => Box::new(bit::res::RES_4_H::new()),
                    (Ok(ImmediateValue::Val8(4)), "l", _) => Box::new(bit::res::RES_4_L::new()),
                    (Ok(ImmediateValue::Val8(4)), "(hl)", _) => {
                        Box::new(bit::res::RES_4_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(4)), "a", _) => Box::new(bit::res::RES_4_A::new()),
                    (Ok(ImmediateValue::Val8(5)), "b", _) => Box::new(bit::res::RES_5_B::new()),
                    (Ok(ImmediateValue::Val8(5)), "c", _) => Box::new(bit::res::RES_5_C::new()),
                    (Ok(ImmediateValue::Val8(5)), "d", _) => Box::new(bit::res::RES_5_D::new()),
                    (Ok(ImmediateValue::Val8(5)), "e", _) => Box::new(bit::res::RES_5_E::new()),
                    (Ok(ImmediateValue::Val8(5)), "h", _) => Box::new(bit::res::RES_5_H::new()),
                    (Ok(ImmediateValue::Val8(5)), "l", _) => Box::new(bit::res::RES_5_L::new()),
                    (Ok(ImmediateValue::Val8(5)), "(hl)", _) => {
                        Box::new(bit::res::RES_5_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(5)), "a", _) => Box::new(bit::res::RES_5_A::new()),
                    (Ok(ImmediateValue::Val8(6)), "b", _) => Box::new(bit::res::RES_6_B::new()),
                    (Ok(ImmediateValue::Val8(6)), "c", _) => Box::new(bit::res::RES_6_C::new()),
                    (Ok(ImmediateValue::Val8(6)), "d", _) => Box::new(bit::res::RES_6_D::new()),
                    (Ok(ImmediateValue::Val8(6)), "e", _) => Box::new(bit::res::RES_6_E::new()),
                    (Ok(ImmediateValue::Val8(6)), "h", _) => Box::new(bit::res::RES_6_H::new()),
                    (Ok(ImmediateValue::Val8(6)), "l", _) => Box::new(bit::res::RES_6_L::new()),
                    (Ok(ImmediateValue::Val8(6)), "(hl)", _) => {
                        Box::new(bit::res::RES_6_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(6)), "a", _) => Box::new(bit::res::RES_6_A::new()),
                    (Ok(ImmediateValue::Val8(7)), "b", _) => Box::new(bit::res::RES_7_B::new()),
                    (Ok(ImmediateValue::Val8(7)), "c", _) => Box::new(bit::res::RES_7_C::new()),
                    (Ok(ImmediateValue::Val8(7)), "d", _) => Box::new(bit::res::RES_7_D::new()),
                    (Ok(ImmediateValue::Val8(7)), "e", _) => Box::new(bit::res::RES_7_E::new()),
                    (Ok(ImmediateValue::Val8(7)), "h", _) => Box::new(bit::res::RES_7_H::new()),
                    (Ok(ImmediateValue::Val8(7)), "l", _) => Box::new(bit::res::RES_7_L::new()),
                    (Ok(ImmediateValue::Val8(7)), "(hl)", _) => {
                        Box::new(bit::res::RES_7_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(7)), "a", _) => Box::new(bit::res::RES_7_A::new()),
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_0_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_1_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_2_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_3_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_4_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_5_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_6_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::res::RES_7_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_0_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_1_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_2_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_3_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_4_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_5_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_6_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::res::RES_7_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid RES operands \"{0}\" and \"{1}\"",
                            bit, destination
                        )))
                    }
                }
            }
            "set" => {
                let bit = get_op(2)?;
                let destination = get_op(3)?;
                match (is_val(bit), destination, is_val(destination)) {
                    (Ok(ImmediateValue::Val8(0)), "b", _) => Box::new(bit::set::SET_0_B::new()),
                    (Ok(ImmediateValue::Val8(0)), "c", _) => Box::new(bit::set::SET_0_C::new()),
                    (Ok(ImmediateValue::Val8(0)), "d", _) => Box::new(bit::set::SET_0_D::new()),
                    (Ok(ImmediateValue::Val8(0)), "e", _) => Box::new(bit::set::SET_0_E::new()),
                    (Ok(ImmediateValue::Val8(0)), "h", _) => Box::new(bit::set::SET_0_H::new()),
                    (Ok(ImmediateValue::Val8(0)), "l", _) => Box::new(bit::set::SET_0_L::new()),
                    (Ok(ImmediateValue::Val8(0)), "(hl)", _) => {
                        Box::new(bit::set::SET_0_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(0)), "a", _) => Box::new(bit::set::SET_0_A::new()),
                    (Ok(ImmediateValue::Val8(1)), "b", _) => Box::new(bit::set::SET_1_B::new()),
                    (Ok(ImmediateValue::Val8(1)), "c", _) => Box::new(bit::set::SET_1_C::new()),
                    (Ok(ImmediateValue::Val8(1)), "d", _) => Box::new(bit::set::SET_1_D::new()),
                    (Ok(ImmediateValue::Val8(1)), "e", _) => Box::new(bit::set::SET_1_E::new()),
                    (Ok(ImmediateValue::Val8(1)), "h", _) => Box::new(bit::set::SET_1_H::new()),
                    (Ok(ImmediateValue::Val8(1)), "l", _) => Box::new(bit::set::SET_1_L::new()),
                    (Ok(ImmediateValue::Val8(1)), "(hl)", _) => {
                        Box::new(bit::set::SET_1_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(1)), "a", _) => Box::new(bit::set::SET_1_A::new()),
                    (Ok(ImmediateValue::Val8(2)), "b", _) => Box::new(bit::set::SET_2_B::new()),
                    (Ok(ImmediateValue::Val8(2)), "c", _) => Box::new(bit::set::SET_2_C::new()),
                    (Ok(ImmediateValue::Val8(2)), "d", _) => Box::new(bit::set::SET_2_D::new()),
                    (Ok(ImmediateValue::Val8(2)), "e", _) => Box::new(bit::set::SET_2_E::new()),
                    (Ok(ImmediateValue::Val8(2)), "h", _) => Box::new(bit::set::SET_2_H::new()),
                    (Ok(ImmediateValue::Val8(2)), "l", _) => Box::new(bit::set::SET_2_L::new()),
                    (Ok(ImmediateValue::Val8(2)), "(hl)", _) => {
                        Box::new(bit::set::SET_2_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(2)), "a", _) => Box::new(bit::set::SET_2_A::new()),
                    (Ok(ImmediateValue::Val8(3)), "b", _) => Box::new(bit::set::SET_3_B::new()),
                    (Ok(ImmediateValue::Val8(3)), "c", _) => Box::new(bit::set::SET_3_C::new()),
                    (Ok(ImmediateValue::Val8(3)), "d", _) => Box::new(bit::set::SET_3_D::new()),
                    (Ok(ImmediateValue::Val8(3)), "e", _) => Box::new(bit::set::SET_3_E::new()),
                    (Ok(ImmediateValue::Val8(3)), "h", _) => Box::new(bit::set::SET_3_H::new()),
                    (Ok(ImmediateValue::Val8(3)), "l", _) => Box::new(bit::set::SET_3_L::new()),
                    (Ok(ImmediateValue::Val8(3)), "(hl)", _) => {
                        Box::new(bit::set::SET_3_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(3)), "a", _) => Box::new(bit::set::SET_3_A::new()),
                    (Ok(ImmediateValue::Val8(4)), "b", _) => Box::new(bit::set::SET_4_B::new()),
                    (Ok(ImmediateValue::Val8(4)), "c", _) => Box::new(bit::set::SET_4_C::new()),
                    (Ok(ImmediateValue::Val8(4)), "d", _) => Box::new(bit::set::SET_4_D::new()),
                    (Ok(ImmediateValue::Val8(4)), "e", _) => Box::new(bit::set::SET_4_E::new()),
                    (Ok(ImmediateValue::Val8(4)), "h", _) => Box::new(bit::set::SET_4_H::new()),
                    (Ok(ImmediateValue::Val8(4)), "l", _) => Box::new(bit::set::SET_4_L::new()),
                    (Ok(ImmediateValue::Val8(4)), "(hl)", _) => {
                        Box::new(bit::set::SET_4_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(4)), "a", _) => Box::new(bit::set::SET_4_A::new()),
                    (Ok(ImmediateValue::Val8(5)), "b", _) => Box::new(bit::set::SET_5_B::new()),
                    (Ok(ImmediateValue::Val8(5)), "c", _) => Box::new(bit::set::SET_5_C::new()),
                    (Ok(ImmediateValue::Val8(5)), "d", _) => Box::new(bit::set::SET_5_D::new()),
                    (Ok(ImmediateValue::Val8(5)), "e", _) => Box::new(bit::set::SET_5_E::new()),
                    (Ok(ImmediateValue::Val8(5)), "h", _) => Box::new(bit::set::SET_5_H::new()),
                    (Ok(ImmediateValue::Val8(5)), "l", _) => Box::new(bit::set::SET_5_L::new()),
                    (Ok(ImmediateValue::Val8(5)), "(hl)", _) => {
                        Box::new(bit::set::SET_5_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(5)), "a", _) => Box::new(bit::set::SET_5_A::new()),
                    (Ok(ImmediateValue::Val8(6)), "b", _) => Box::new(bit::set::SET_6_B::new()),
                    (Ok(ImmediateValue::Val8(6)), "c", _) => Box::new(bit::set::SET_6_C::new()),
                    (Ok(ImmediateValue::Val8(6)), "d", _) => Box::new(bit::set::SET_6_D::new()),
                    (Ok(ImmediateValue::Val8(6)), "e", _) => Box::new(bit::set::SET_6_E::new()),
                    (Ok(ImmediateValue::Val8(6)), "h", _) => Box::new(bit::set::SET_6_H::new()),
                    (Ok(ImmediateValue::Val8(6)), "l", _) => Box::new(bit::set::SET_6_L::new()),
                    (Ok(ImmediateValue::Val8(6)), "(hl)", _) => {
                        Box::new(bit::set::SET_6_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(6)), "a", _) => Box::new(bit::set::SET_6_A::new()),
                    (Ok(ImmediateValue::Val8(7)), "b", _) => Box::new(bit::set::SET_7_B::new()),
                    (Ok(ImmediateValue::Val8(7)), "c", _) => Box::new(bit::set::SET_7_C::new()),
                    (Ok(ImmediateValue::Val8(7)), "d", _) => Box::new(bit::set::SET_7_D::new()),
                    (Ok(ImmediateValue::Val8(7)), "e", _) => Box::new(bit::set::SET_7_E::new()),
                    (Ok(ImmediateValue::Val8(7)), "h", _) => Box::new(bit::set::SET_7_H::new()),
                    (Ok(ImmediateValue::Val8(7)), "l", _) => Box::new(bit::set::SET_7_L::new()),
                    (Ok(ImmediateValue::Val8(7)), "(hl)", _) => {
                        Box::new(bit::set::SET_7_PHL::new())
                    }
                    (Ok(ImmediateValue::Val8(7)), "a", _) => Box::new(bit::set::SET_7_A::new()),
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_0_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_1_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_2_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_3_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_4_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_5_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_6_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIX(offset))) => {
                        Box::new(bit::set::SET_7_PIXD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(0)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_0_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(1)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_1_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(2)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_2_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(3)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_3_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(4)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_4_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(5)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_5_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(6)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_6_PIYD::new_with_value(offset))
                    }
                    (Ok(ImmediateValue::Val8(7)), _, Ok(ImmediateValue::OffsetIY(offset))) => {
                        Box::new(bit::set::SET_7_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid SET operands \"{0}\" and \"{1}\"",
                            bit, destination
                        )))
                    }
                }
            }
            "ld" => {
                let destination = get_op(2)?;
                let source = get_op(3)?;
                match (is_val(destination), is_val(source)) {
                    (Err(_), Ok(ImmediateValue::Val8(val))) => match destination {
                        "b" => Box::new(ld::LD_B_N::new_with_value(val)),
                        "c" => Box::new(ld::LD_C_N::new_with_value(val)),
                        "d" => Box::new(ld::LD_D_N::new_with_value(val)),
                        "e" => Box::new(ld::LD_E_N::new_with_value(val)),
                        "h" => Box::new(ld::LD_H_N::new_with_value(val)),
                        "l" => Box::new(ld::LD_L_N::new_with_value(val)),
                        "a" => Box::new(ld::LD_A_N::new_with_value(val)),
                        "(hl)" => Box::new(ld::LD_PHL_N::new_with_value(val)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::Val16(val))) => match destination {
                        "bc" => Box::new(ld::LD_BC_NN::new_with_value(val)),
                        "de" => Box::new(ld::LD_DE_NN::new_with_value(val)),
                        "hl" => Box::new(ld::LD_HL_NN::new_with_value(val)),
                        "sp" => Box::new(ld::ld_sp_nn::LD_SP_NN::new_with_value(val)),
                        "ix" => Box::new(ld::ld_ix_nn::LD_IX_NN::new_with_value(val)),
                        "iy" => Box::new(ld::ld_iy_nn::LD_IY_NN::new_with_value(val)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::Ptr(val))) => match destination {
                        "a" => Box::new(ld::LD_A_PNN::new_with_value(val)),
                        "bc" => Box::new(ld::LD_MISC_BC_PNN::new_with_value(val)),
                        "de" => Box::new(ld::LD_MISC_DE_PNN::new_with_value(val)),
                        "hl" => Box::new(ld::LD_MISC_HL_PNN::new_with_value(val)),
                        "sp" => Box::new(ld::ld_sp_pnn::LD_MISC_SP_PNN::new_with_value(val)),
                        "ix" => Box::new(ld::ld_ix_pnn::LD_IX_PNN::new_with_value(val)),
                        "iy" => Box::new(ld::ld_iy_pnn::LD_IY_PNN::new_with_value(val)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },

                    (Ok(ImmediateValue::Val16(_)), Err(_)) => match source {
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid source \"{0}\"",
                                source
                            )))
                        }
                    },
                    (Ok(ImmediateValue::Ptr(val)), Err(_)) => match source {
                        "ix" => Box::new(ld::ld_pnn_ix::LD_PNN_IX::new_with_value(val)),
                        "iy" => Box::new(ld::ld_pnn_iy::LD_PNN_IY::new_with_value(val)),
                        "hl" => Box::new(ld::LD_PNN_HL::new_with_value(val)),
                        "bc" => Box::new(ld::ld_pnn_bc_misc::LD_PNN_BC::new_with_value(val)),
                        "de" => Box::new(ld::ld_pnn_de_misc::LD_PNN_DE::new_with_value(val)),
                        "sp" => Box::new(ld::ld_pnn_sp::LD_PNN_SP::new_with_value(val)),
                        "a" => Box::new(ld::LD_PNN_A::new_with_value(val)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid source \"{0}\"",
                                source
                            )))
                        }
                    },
                    (Ok(ImmediateValue::OffsetIX(offset)), Ok(ImmediateValue::Val8(val))) => {
                        Box::new(ld::ld_pixd_n::LD_PIXD_N::new_with_value(offset, val))
                    }
                    (Ok(ImmediateValue::OffsetIX(offset)), Err(_)) => match source {
                        "b" => Box::new(ld::LD_PIXD_B::new_with_value(offset)),
                        "c" => Box::new(ld::LD_PIXD_C::new_with_value(offset)),
                        "d" => Box::new(ld::LD_PIXD_D::new_with_value(offset)),
                        "e" => Box::new(ld::LD_PIXD_E::new_with_value(offset)),
                        "h" => Box::new(ld::LD_PIXD_H::new_with_value(offset)),
                        "l" => Box::new(ld::LD_PIXD_L::new_with_value(offset)),
                        "a" => Box::new(ld::LD_PIXD_A::new_with_value(offset)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid source \"{0}\"",
                                source
                            )))
                        }
                    },
                    (Ok(ImmediateValue::OffsetIY(offset)), Err(_)) => match source {
                        "b" => Box::new(ld::LD_PIYD_B::new_with_value(offset)),
                        "c" => Box::new(ld::LD_PIYD_C::new_with_value(offset)),
                        "d" => Box::new(ld::LD_PIYD_D::new_with_value(offset)),
                        "e" => Box::new(ld::LD_PIYD_E::new_with_value(offset)),
                        "h" => Box::new(ld::LD_PIYD_H::new_with_value(offset)),
                        "l" => Box::new(ld::LD_PIYD_L::new_with_value(offset)),
                        "a" => Box::new(ld::LD_PIYD_A::new_with_value(offset)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid source \"{0}\"",
                                source
                            )))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::OffsetIX(offset))) => match destination {
                        "a" => Box::new(ld::LD_A_PIXD::new_with_value(offset)),
                        "b" => Box::new(ld::LD_B_PIXD::new_with_value(offset)),
                        "c" => Box::new(ld::LD_C_PIXD::new_with_value(offset)),
                        "d" => Box::new(ld::LD_D_PIXD::new_with_value(offset)),
                        "e" => Box::new(ld::LD_E_PIXD::new_with_value(offset)),
                        "h" => Box::new(ld::LD_H_PIXD::new_with_value(offset)),
                        "l" => Box::new(ld::LD_L_PIXD::new_with_value(offset)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::OffsetIY(offset))) => match destination {
                        "a" => Box::new(ld::LD_A_PIYD::new_with_value(offset)),
                        "b" => Box::new(ld::LD_B_PIYD::new_with_value(offset)),
                        "c" => Box::new(ld::LD_C_PIYD::new_with_value(offset)),
                        "d" => Box::new(ld::LD_D_PIYD::new_with_value(offset)),
                        "e" => Box::new(ld::LD_E_PIYD::new_with_value(offset)),
                        "h" => Box::new(ld::LD_H_PIYD::new_with_value(offset)),
                        "l" => Box::new(ld::LD_L_PIYD::new_with_value(offset)),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },
                    (Err(_), Err(_)) => match (destination, source) {
                        ("(bc)", "a") => Box::new(ld::LD_PBC_A::new()),
                        ("(de)", "a") => Box::new(ld::LD_PDE_A::new()),
                        ("(hl)", "a") => Box::new(ld::LD_PHL_A::new()),
                        ("(hl)", "b") => Box::new(ld::LD_PHL_B::new()),
                        ("(hl)", "c") => Box::new(ld::LD_PHL_C::new()),
                        ("(hl)", "d") => Box::new(ld::LD_PHL_D::new()),
                        ("(hl)", "e") => Box::new(ld::LD_PHL_E::new()),
                        ("(hl)", "h") => Box::new(ld::LD_PHL_H::new()),
                        ("(hl)", "l") => Box::new(ld::LD_PHL_L::new()),
                        ("a", "(bc)") => Box::new(ld::LD_A_PBC::new()),
                        ("a", "(hl)") => Box::new(ld::LD_A_PHL::new()),
                        ("a", "(de)") => Box::new(ld::LD_A_PDE::new()),
                        ("a", "a") => Box::new(ld::LD_A_A::new()),
                        ("a", "b") => Box::new(ld::LD_A_B::new()),
                        ("a", "c") => Box::new(ld::LD_A_C::new()),
                        ("a", "d") => Box::new(ld::LD_A_D::new()),
                        ("a", "e") => Box::new(ld::LD_A_E::new()),
                        ("a", "h") => Box::new(ld::LD_A_H::new()),
                        ("a", "l") => Box::new(ld::LD_A_L::new()),

                        ("b", "(hl)") => Box::new(ld::LD_B_PHL::new()),
                        ("b", "b") => Box::new(ld::LD_B_B::new()),
                        ("b", "c") => Box::new(ld::LD_B_C::new()),
                        ("b", "d") => Box::new(ld::LD_B_D::new()),
                        ("b", "e") => Box::new(ld::LD_B_E::new()),
                        ("b", "h") => Box::new(ld::LD_B_H::new()),
                        ("b", "l") => Box::new(ld::LD_B_L::new()),
                        ("b", "a") => Box::new(ld::LD_B_A::new()),

                        ("c", "c") => Box::new(ld::LD_C_C::new()),
                        ("c", "b") => Box::new(ld::LD_C_B::new()),
                        ("c", "d") => Box::new(ld::LD_C_D::new()),
                        ("c", "e") => Box::new(ld::LD_C_E::new()),
                        ("c", "h") => Box::new(ld::LD_C_H::new()),
                        ("c", "l") => Box::new(ld::LD_C_L::new()),
                        ("c", "(hl)") => Box::new(ld::LD_C_PHL::new()),
                        ("c", "a") => Box::new(ld::LD_C_A::new()),

                        ("d", "c") => Box::new(ld::LD_D_C::new()),
                        ("d", "d") => Box::new(ld::LD_D_D::new()),
                        ("d", "b") => Box::new(ld::LD_D_B::new()),
                        ("d", "e") => Box::new(ld::LD_D_E::new()),
                        ("d", "h") => Box::new(ld::LD_D_H::new()),
                        ("d", "l") => Box::new(ld::LD_D_L::new()),
                        ("d", "(hl)") => Box::new(ld::LD_D_PHL::new()),
                        ("d", "a") => Box::new(ld::LD_D_A::new()),

                        ("e", "b") => Box::new(ld::LD_E_B::new()),
                        ("e", "c") => Box::new(ld::LD_E_C::new()),
                        ("e", "d") => Box::new(ld::LD_E_D::new()),
                        ("e", "e") => Box::new(ld::LD_E_E::new()),
                        ("e", "h") => Box::new(ld::LD_E_H::new()),
                        ("e", "l") => Box::new(ld::LD_E_L::new()),
                        ("e", "(hl)") => Box::new(ld::LD_E_PHL::new()),
                        ("e", "a") => Box::new(ld::LD_E_A::new()),

                        ("h", "b") => Box::new(ld::LD_H_B::new()),
                        ("h", "c") => Box::new(ld::LD_H_C::new()),
                        ("h", "d") => Box::new(ld::LD_H_D::new()),
                        ("h", "e") => Box::new(ld::LD_H_E::new()),
                        ("h", "h") => Box::new(ld::LD_H_H::new()),
                        ("h", "l") => Box::new(ld::LD_H_L::new()),
                        ("h", "(hl)") => Box::new(ld::LD_H_PHL::new()),
                        ("h", "a") => Box::new(ld::LD_H_A::new()),

                        ("l", "b") => Box::new(ld::LD_L_B::new()),
                        ("l", "c") => Box::new(ld::LD_L_C::new()),
                        ("l", "d") => Box::new(ld::LD_L_D::new()),
                        ("l", "e") => Box::new(ld::LD_L_E::new()),
                        ("l", "h") => Box::new(ld::LD_L_H::new()),
                        ("l", "l") => Box::new(ld::LD_L_L::new()),
                        ("l", "(hl)") => Box::new(ld::LD_L_PHL::new()),
                        ("l", "a") => Box::new(ld::LD_L_A::new()),

                        ("i","a") => Box::new(ld::ld_i_a::LD_I_A::new()),
                        ("a","i") => Box::new(ld::ld_a_i::LD_A_I::new()),

                        ("sp", "hl") => Box::new(ld::ld_sp_hl::LD_SP_HL::new()),
                        ("sp", "ix") => Box::new(ld::ld_sp_ix::LD_SP_IX::new()),
                        ("sp", "iy") => Box::new(ld::ld_sp_iy::LD_SP_IY::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operands \"{0}\" and \"{1}\"",
                                source, destination
                            )))
                        }
                    },
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operands \"{0}\" and \"{1}\"",
                            source, destination
                        )))
                    }
                }
            }
            "inc" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::inc::inc_pixd::INC_PIXD::new_with_value(offset as i8))
                    }
                    Err(_) => match destination {
                        "bc" => Box::new(math::inc::INC_BC::new()),
                        "de" => Box::new(math::inc::INC_DE::new()),
                        "hl" => Box::new(math::inc::INC_HL::new()),
                        "sp" => Box::new(math::inc::inc_sp::INC_SP::new()),
                        "b" => Box::new(math::inc::INC_B::new()),
                        "c" => Box::new(math::inc::INC_C::new()),
                        "d" => Box::new(math::inc::INC_D::new()),
                        "e" => Box::new(math::inc::INC_E::new()),
                        "h" => Box::new(math::inc::INC_H::new()),
                        "l" => Box::new(math::inc::INC_L::new()),
                        "a" => Box::new(math::inc::INC_A::new()),
                        "(hl)" => Box::new(math::inc::inc_phl::INC_PHL::new()),
                        "ix" => Box::new(math::inc::inc_ix::INC_IX::new()),
                        "iy" => Box::new(math::inc::inc_iy::INC_IY::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operand \"{0}\"",
                                destination
                            )))
                        }
                    },
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operand \"{0}\"",
                            destination
                        )))
                    }
                }
            }
            "dec" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::dec::dec_pixd::DEC_PIXD::new_with_value(offset as i8))
                    }
                    Err(_) => match destination {
                        "bc" => Box::new(math::dec::DEC_BC::new()),
                        "de" => Box::new(math::dec::DEC_DE::new()),
                        "hl" => Box::new(math::dec::DEC_HL::new()),
                        "sp" => Box::new(math::dec::dec_sp::DEC_SP::new()),
                        "b" => Box::new(math::dec::DEC_B::new()),
                        "c" => Box::new(math::dec::DEC_C::new()),
                        "d" => Box::new(math::dec::DEC_D::new()),
                        "e" => Box::new(math::dec::DEC_E::new()),
                        "h" => Box::new(math::dec::DEC_H::new()),
                        "l" => Box::new(math::dec::DEC_L::new()),
                        "a" => Box::new(math::dec::DEC_A::new()),
                        "(hl)" => Box::new(math::dec::dec_phl::DEC_PHL::new()),
                        "ix" => Box::new(math::dec::dec_ix::DEC_IX::new()),
                        "iy" => Box::new(math::dec::dec_iy::DEC_IY::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operand \"{0}\"",
                                destination
                            )))
                        }
                    },
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operand \"{0}\"",
                            destination
                        )))
                    }
                }
            }
            "add" => {
                let destination = get_op(2)?;
                match destination {
                    "hl" => {
                        let source = get_op(3)?;
                        match source {
                            "bc" => Box::new(math::add::ADD_HL_BC::new()),
                            "de" => Box::new(math::add::ADD_HL_DE::new()),
                            "hl" => Box::new(math::add::ADD_HL_HL::new()),
                            "sp" => Box::new(math::add::add_hl_sp::ADD_HL_SP::new()),
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid source \"{0}\"",
                                    source
                                )))
                            }
                        }
                    }
                    "ix" => {
                        let source = get_op(3)?;
                        match source {
                            "bc" => Box::new(math::add::add_ix_bc::ADD_IX_BC::new()),
                            "de" => Box::new(math::add::add_ix_de::ADD_IX_DE::new()),
                            "sp" => Box::new(math::add::add_ix_sp::ADD_IX_SP::new()),
                            "ix" => Box::new(math::add::add_ix_ix::ADD_IX_IX::new()),
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid source \"{0}\"",
                                    source
                                )))
                            }
                        }
                    }
                    "iy" => {
                        let source = get_op(3)?;
                        match source {
                            "bc" => Box::new(math::add::add_iy_bc::ADD_IY_BC::new()),
                            "de" => Box::new(math::add::add_iy_de::ADD_IY_DE::new()),
                            "sp" => Box::new(math::add::add_iy_sp::ADD_IY_SP::new()),
                            "iy" => Box::new(math::add::add_iy_iy::ADD_IY_IY::new()),
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid source \"{0}\"",
                                    source
                                )))
                            }
                        }
                    }
                    "a" => {
                        let source = get_op(3)?;
                        match is_val(source) {
                            Err(_) => match source {
                                "a" => Box::new(math::add::ADD_A_A::new()),
                                "b" => Box::new(math::add::ADD_A_B::new()),
                                "c" => Box::new(math::add::ADD_A_C::new()),
                                "d" => Box::new(math::add::ADD_A_D::new()),
                                "e" => Box::new(math::add::ADD_A_E::new()),
                                "h" => Box::new(math::add::ADD_A_H::new()),
                                "l" => Box::new(math::add::ADD_A_L::new()),
                                "(hl)" => Box::new(math::add::add_a_phl::ADD_A_PHL::new()),
                                _ => {
                                    return Err(ParseError::InvalidInstruction(format!(
                                        "Invalid source \"{0}\"",
                                        source
                                    )))
                                }
                            },
                            Ok(ImmediateValue::OffsetIX(offset)) => {
                                Box::new(math::add::add_a_pixd::ADD_A_PIXD::new_with_value(offset))
                            }
                            Ok(ImmediateValue::OffsetIY(offset)) => {
                                Box::new(math::add::add_a_piyd::ADD_A_PIYD::new_with_value(offset))
                            }
                            Ok(ImmediateValue::Val8(val)) => {
                                Box::new(math::add::add_a_n::ADD_A_N::new_with_value(val))
                            }
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid source \"{0}\"",
                                    source
                                )))
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid destination \"{0}\"",
                            destination
                        )))
                    }
                }
            }
            "adc" => {
                let destination = get_op(2)?;
                match destination {
                    "a" => {
                        let source = get_op(3)?;
                        match is_val(source) {
                            Ok(ImmediateValue::Val8(val)) => {
                                Box::new(math::adc::adc_a_n::ADC_A_N::new_with_value(val))
                            }
                            Ok(ImmediateValue::OffsetIX(offset)) => {
                                Box::new(math::adc::adc_a_pixd::ADC_A_PIXD::new_with_value(offset))
                            }
                            Ok(ImmediateValue::OffsetIY(offset)) => {
                                Box::new(math::adc::adc_a_piyd::ADC_A_PIYD::new_with_value(offset))
                            }
                            _ => match source {
                                "a" => Box::new(math::adc::ADC_A_A::new()),
                                "b" => Box::new(math::adc::ADC_A_B::new()),
                                "c" => Box::new(math::adc::ADC_A_C::new()),
                                "d" => Box::new(math::adc::ADC_A_D::new()),
                                "e" => Box::new(math::adc::ADC_A_E::new()),
                                "h" => Box::new(math::adc::ADC_A_H::new()),
                                "l" => Box::new(math::adc::ADC_A_L::new()),
                                "(hl)" => Box::new(math::adc::adc_a_phl::ADC_A_PHL::new()),
                                _ => {
                                    return Err(ParseError::InvalidInstruction(format!(
                                        "Invalid source \"{0}\"",
                                        source
                                    )))
                                }
                            },
                        }
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid destination \"{0}\"",
                            destination
                        )))
                    }
                }
            }
            "sbc" => {
                let destination = get_op(2)?;
                let source = get_op(3)?;
                match (destination, is_val(source), source) {
                    ("a", Ok(ImmediateValue::Val8(val)), _) => {
                        Box::new(math::sbc::sbc_a_n::SBC_A_N::new_with_value(val))
                    }
                    ("a", Ok(ImmediateValue::OffsetIX(offset)), _) => {
                        Box::new(math::sbc::sbc_a_pixd::SBC_A_PIXD::new_with_value(offset))
                    }
                    ("a", Ok(ImmediateValue::OffsetIY(offset)), _) => {
                        Box::new(math::sbc::sbc_a_piyd::SBC_A_PIYD::new_with_value(offset))
                    }
                    ("a", Err(_), "a") => Box::new(math::sbc::SBC_A_A::new()),
                    ("a", Err(_), "b") => Box::new(math::sbc::SBC_A_B::new()),
                    ("a", Err(_), "c") => Box::new(math::sbc::SBC_A_C::new()),
                    ("a", Err(_), "d") => Box::new(math::sbc::SBC_A_D::new()),
                    ("a", Err(_), "e") => Box::new(math::sbc::SBC_A_E::new()),
                    ("a", Err(_), "h") => Box::new(math::sbc::SBC_A_H::new()),
                    ("a", Err(_), "l") => Box::new(math::sbc::SBC_A_L::new()),
                    ("a", Err(_), "(hl)") => Box::new(math::sbc::sbc_a_phl::SBC_A_PHL::new()),
                    ("hl", Err(_), "bc") => Box::new(math::sbc::sbc_hl_bc::SBC_HL_BC::new()),
                    ("hl", _, "de") => Box::new(math::sbc::sbc_hl_de::SBC_HL_DE::new()),
                    ("hl", _, "hl") => Box::new(math::sbc::sbc_hl_hl::SBC_HL_HL::new()),
                    ("hl", _, "sp") => Box::new(math::sbc::sbc_hl_sp::SBC_HL_SP::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operands \"{0}\" and \"{1}\"",
                            destination, source
                        )))
                    }
                }
            }
            "xor" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::Val8(val)) => {
                        Box::new(math::xor::xor_n::XOR_N::new_with_value(val))
                    }
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::xor::xor_pixd::XOR_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(math::xor::xor_piyd::XOR_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "a" => Box::new(math::xor::XOR_A::new()),
                        "b" => Box::new(math::xor::XOR_B::new()),
                        "c" => Box::new(math::xor::XOR_C::new()),
                        "d" => Box::new(math::xor::XOR_D::new()),
                        "e" => Box::new(math::xor::XOR_E::new()),
                        "h" => Box::new(math::xor::XOR_H::new()),
                        "l" => Box::new(math::xor::XOR_L::new()),
                        "(hl)" => Box::new(math::xor::xor_phl::XOR_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid destination \"{0}\"",
                                destination
                            )))
                        }
                    },
                }
            }
            "rlca" => Box::new(rlca::RLCA::new()),
            "ex" => {
                let op1 = get_op(2)?;
                let op2 = get_op(3)?;
                match (op1, op2) {
                    ("de", "hl") => Box::new(ex::ex_de_hl::EX_DE_HL::new()),
                    ("af", "af'") => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
                    ("(sp)", "hl") => Box::new(ex::ex_psp_hl::EX_PSP_HL::new()),
                    ("(sp)", "ix") => Box::new(ex::ex_psp_ix::EX_PSP_IX::new()),
                    ("(sp)", "iy") => Box::new(ex::ex_psp_iy::EX_PSP_IY::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operands \"{0}\" and \"{1}\"",
                            op1, op2
                        )))
                    }
                }
            }
            "cp" => {
                let operator = get_op(2)?;
                match is_val(operator) {
                    Err(_) => match operator {
                        "a" => Box::new(math::cp::CP_A::new()),
                        "b" => Box::new(math::cp::CP_B::new()),
                        "c" => Box::new(math::cp::CP_C::new()),
                        "d" => Box::new(math::cp::CP_D::new()),
                        "e" => Box::new(math::cp::CP_E::new()),
                        "h" => Box::new(math::cp::CP_H::new()),
                        "l" => Box::new(math::cp::CP_L::new()),
                        "(hl)" => Box::new(math::cp::cp_phl::CP_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operator \"{0}\"",
                                operator
                            )))
                        }
                    },
                    Ok(ImmediateValue::Val8(val)) => {
                        Box::new(math::cp::cp_n::CP_N::new_with_value(val))
                    }
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::cp::cp_pixd::CP_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(math::cp::cp_piyd::CP_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operator \"{0}\"",
                            operator
                        )))
                    }
                }
            }
            "and" => {
                let operator = get_op(2)?;
                match is_val(operator) {
                    Err(_) => match operator {
                        "a" => Box::new(math::and::AND_A::new()),
                        "b" => Box::new(math::and::AND_B::new()),
                        "c" => Box::new(math::and::AND_C::new()),
                        "d" => Box::new(math::and::AND_D::new()),
                        "e" => Box::new(math::and::AND_E::new()),
                        "h" => Box::new(math::and::AND_H::new()),
                        "l" => Box::new(math::and::AND_L::new()),
                        "(hl)" => Box::new(math::and::and_phl::AND_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operator \"{0}\"",
                                operator
                            )))
                        }
                    },
                    Ok(ImmediateValue::Val8(val)) => {
                        Box::new(math::and::and_n::AND_N::new_with_value(val))
                    }
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::and::and_pixd::AND_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(math::and::and_piyd::AND_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operator \"{0}\"",
                            operator
                        )))
                    }
                }
            }
            "sub" => {
                let operator = get_op(2)?;
                match is_val(operator) {
                    Err(_) => match operator {
                        "a" => Box::new(math::sub::SUB_A::new()),
                        "b" => Box::new(math::sub::SUB_B::new()),
                        "c" => Box::new(math::sub::SUB_C::new()),
                        "d" => Box::new(math::sub::SUB_D::new()),
                        "e" => Box::new(math::sub::SUB_E::new()),
                        "h" => Box::new(math::sub::SUB_H::new()),
                        "l" => Box::new(math::sub::SUB_L::new()),
                        "(hl)" => Box::new(math::sub::sub_phl::SUB_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operator \"{0}\"",
                                operator
                            )))
                        }
                    },
                    Ok(ImmediateValue::Val8(val)) => {
                        Box::new(math::sub::sub_n::SUB_N::new_with_value(val))
                    }
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::sub::sub_pixd::SUB_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(math::sub::sub_piyd::SUB_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operator \"{0}\"",
                            operator
                        )))
                    }
                }
            }
            "or" => {
                let operator = get_op(2)?;
                match is_val(operator) {
                    Err(_) => match operator {
                        "a" => Box::new(math::or::OR_A::new()),
                        "b" => Box::new(math::or::OR_B::new()),
                        "c" => Box::new(math::or::OR_C::new()),
                        "d" => Box::new(math::or::OR_D::new()),
                        "e" => Box::new(math::or::OR_E::new()),
                        "h" => Box::new(math::or::OR_H::new()),
                        "l" => Box::new(math::or::OR_L::new()),
                        "(hl)" => Box::new(math::or::or_phl::OR_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(format!(
                                "Invalid operator \"{0}\"",
                                operator
                            )))
                        }
                    },
                    Ok(ImmediateValue::Val8(val)) => {
                        Box::new(math::or::or_n::OR_N::new_with_value(val))
                    }
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(math::or::or_pixd::OR_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(math::or::or_piyd::OR_PIYD::new_with_value(offset))
                    }
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operator \"{0}\"",
                            operator
                        )))
                    }
                }
            }
            "rrca" => Box::new(rrca::RRCA::new()),
            "djnz" => {
                let operator = get_op(2)?;
                match is_val(operator) {
                    Ok(ImmediateValue::Val8(val)) => Box::new(djnz_d::DJNZ_D::new_with_value(val)),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid operator \"{0}\"",
                            operator
                        )))
                    }
                }
            }
            "rla" => Box::new(rla::RLA::new()),
            "jr" => {
                let op1 = get_op(2)?;
                let op2 = get_op(3);
                if let Ok(op2_str) = op2 {
                    match (is_val(op1), is_val(op2_str)) {
                        (Err(_), Ok(ImmediateValue::Val8(val))) => match op1 {
                            "z" => Box::new(jump::jr::jr_z_d::JR_Z_D::new_with_value(val)),
                            "nz" => Box::new(jump::jr::jr_nz_d::JR_NZ_D::new_with_value(val)),
                            "nc" => Box::new(jump::jr::jr_nc_d::JR_NC_D::new_with_value(val)),
                            "c" => Box::new(jump::jr::jr_c_d::JR_C_D::new_with_value(val)),
                            _ => {
                                return Err(ParseError::InvalidInstruction(
                                    "Invalid instruction".to_string(),
                                ))
                            }
                        },

                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    }
                } else if let Ok(ImmediateValue::Val8(val)) = is_val(op1) {
                    Box::new(jump::jr::jr_d::JR_D::new_with_value(val))
                } else {
                    return Err(ParseError::InvalidInstruction(
                        "Invalid instruction".to_string(),
                    ));
                }
            }
            "jp" => {
                let op1 = get_op(2)?;
                let op2 = get_op(3);
                if let Ok(op2_str) = op2 {
                    match (is_val(op1), is_val(op2_str)) {
                        (Err(_), Ok(ImmediateValue::Val16(val))) => match op1 {
                            "nz" => Box::new(jump::jp::jp_nz_nn::JP_NZ_NN::new_with_value(val)),
                            "z" => Box::new(jump::jp::jp_z_nn::JP_Z_NN::new_with_value(val)),
                            "nc" => Box::new(jump::jp::jp_nc_nn::JP_NC_NN::new_with_value(val)),
                            "c" => Box::new(jump::jp::jp_c_nn::JP_C_NN::new_with_value(val)),
                            "po" => Box::new(jump::jp::jp_po_nn::JP_PO_NN::new_with_value(val)),
                            "pe" => Box::new(jump::jp::jp_pe_nn::JP_PE_NN::new_with_value(val)),
                            "p" => Box::new(jump::jp::jp_p_nn::JP_P_NN::new_with_value(val)),
                            "m" => Box::new(jump::jp::jp_m_nn::JP_M_NN::new_with_value(val)),
                            _ => {
                                return Err(ParseError::InvalidInstruction(
                                    "Invalid instruction".to_string(),
                                ))
                            }
                        },
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    }
                } else if let Ok(ImmediateValue::Val16(val)) = is_val(op1) {
                    Box::new(jump::jp::jp_nn::JP_NN::new_with_value(val))
                } else if op1 == "(hl)" {
                    Box::new(jump::jp::jp_phl::JP_PHL::new())
                } else if op1 == "(ix)" {
                    Box::new(jump::jp::jp_pix::JP_PIX::new())
                } else if op1 == "(iy)" {
                    Box::new(jump::jp::jp_piy::JP_PIY::new())
                } else {
                    return Err(ParseError::InvalidInstruction(
                        "Invalid instruction".to_string(),
                    ));
                }
            }
            "rst" => {
                let destination = get_op(2)?;
                if let Ok(ImmediateValue::Val8(val)) = is_val(destination) {
                    match val {
                        0x00 => Box::new(rst::RST_0x00::new()),
                        0x08 => Box::new(rst::RST_0x08::new()),
                        0x10 => Box::new(rst::RST_0x10::new()),
                        0x18 => Box::new(rst::RST_0x18::new()),
                        0x20 => Box::new(rst::RST_0x20::new()),
                        0x28 => Box::new(rst::RST_0x28::new()),
                        0x30 => Box::new(rst::RST_0x30::new()),
                        0x38 => Box::new(rst::RST_0x38::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Bad RST address".to_string(),
                            ))
                        }
                    }
                } else {
                    return Err(ParseError::InvalidInstruction(
                        "Bad RST parameter".to_string(),
                    ));
                }
            }
            "rra" => Box::new(rra::RRA::new()),
            "halt" => Box::new(halt::Halt::new()),
            "call" => {
                let op1 = get_op(2)?;
                let op2 = get_op(3);
                if let Ok(op2_str) = op2 {
                    match (is_val(op1), is_val(op2_str)) {
                        (Err(_), Ok(ImmediateValue::Val16(val))) => match op1 {
                            "nz" => Box::new(call::call_nz_nn::CALL_NZ_NN::new_with_value(val)),
                            "z" => Box::new(call::call_z_nn::CALL_Z_NN::new_with_value(val)),
                            "nc" => Box::new(call::call_nc_nn::CALL_NC_NN::new_with_value(val)),
                            "c" => Box::new(call::call_c_nn::CALL_C_NN::new_with_value(val)),
                            "po" => Box::new(call::call_po_nn::CALL_PO_NN::new_with_value(val)),
                            "pe" => Box::new(call::call_pe_nn::CALL_PE_NN::new_with_value(val)),
                            "p" => Box::new(call::call_p_nn::CALL_P_NN::new_with_value(val)),
                            "m" => Box::new(call::call_m_nn::CALL_M_NN::new_with_value(val)),
                            _ => {
                                return Err(ParseError::InvalidInstruction(
                                    "Invalid instruction".to_string(),
                                ))
                            }
                        },
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    }
                } else if let Ok(ImmediateValue::Val16(val)) = is_val(op1) {
                    Box::new(call::call_nn::CALL_NN::new_with_value(val))
                } else {
                    return Err(ParseError::InvalidInstruction(
                        "Invalid instruction".to_string(),
                    ));
                }
            }
            "ret" => {
                let condition = get_op(2);
                if let Ok(condition) = condition {
                    match condition {
                        "nz" => Box::new(ret::ret_nz::RET_NZ::new()),
                        "z" => Box::new(ret::ret_z::RET_Z::new()),
                        "nc" => Box::new(ret::ret_nc::RET_NC::new()),
                        "c" => Box::new(ret::ret_c::RET_C::new()),
                        "po" => Box::new(ret::ret_po::RET_PO::new()),
                        "pe" => Box::new(ret::ret_pe::RET_PE::new()),
                        "p" => Box::new(ret::ret_p::RET_P::new()),
                        "m" => Box::new(ret::ret_m::RET_M::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    }
                } else {
                    Box::new(ret::ret::RET::new())
                }
            }
            "push" => {
                let destination = get_op(2)?;
                match destination {
                    "af" => Box::new(stack::push::PUSH_AF::new()),
                    "bc" => Box::new(stack::push::PUSH_BC::new()),
                    "de" => Box::new(stack::push::PUSH_DE::new()),
                    "hl" => Box::new(stack::push::PUSH_HL::new()),
                    "ix" => Box::new(stack::push::push_ix::PUSH_IX::new()),
                    "iy" => Box::new(stack::push::push_iy::PUSH_IY::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(
                            "Invalid instruction".to_string(),
                        ))
                    }
                }
            }
            "pop" => {
                let destination = get_op(2)?;
                match destination {
                    "af" => Box::new(stack::pop::POP_AF::new()),
                    "bc" => Box::new(stack::pop::POP_BC::new()),
                    "de" => Box::new(stack::pop::POP_DE::new()),
                    "hl" => Box::new(stack::pop::POP_HL::new()),
                    "ix" => Box::new(stack::pop::pop_ix::POP_IX::new()),
                    "iy" => Box::new(stack::pop::pop_iy::POP_IY::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(
                            "Invalid instruction".to_string(),
                        ))
                    }
                }
            }
            "rr" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::rr::rr_pixd::RR_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::rr::rr_piyd::RR_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::rr::RR_B::new()),
                        "c" => Box::new(bit::rr::RR_C::new()),
                        "d" => Box::new(bit::rr::RR_D::new()),
                        "e" => Box::new(bit::rr::RR_E::new()),
                        "h" => Box::new(bit::rr::RR_H::new()),
                        "l" => Box::new(bit::rr::RR_L::new()),
                        "a" => Box::new(bit::rr::RR_A::new()),
                        "(hl)" => Box::new(bit::rr::rr_phl::RR_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "rlc" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::rlc::rlc_pixd::RLC_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::rlc::rlc_piyd::RLC_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::rlc::RLC_B::new()),
                        "c" => Box::new(bit::rlc::RLC_C::new()),
                        "d" => Box::new(bit::rlc::RLC_D::new()),
                        "e" => Box::new(bit::rlc::RLC_E::new()),
                        "h" => Box::new(bit::rlc::RLC_H::new()),
                        "l" => Box::new(bit::rlc::RLC_L::new()),
                        "a" => Box::new(bit::rlc::RLC_A::new()),
                        "(hl)" => Box::new(bit::rlc::rlc_phl::RLC_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "rrc" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::rrc::rrc_pixd::RRC_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::rrc::rrc_piyd::RRC_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::rrc::RRC_B::new()),
                        "c" => Box::new(bit::rrc::RRC_C::new()),
                        "d" => Box::new(bit::rrc::RRC_D::new()),
                        "e" => Box::new(bit::rrc::RRC_E::new()),
                        "h" => Box::new(bit::rrc::RRC_H::new()),
                        "l" => Box::new(bit::rrc::RRC_L::new()),
                        "a" => Box::new(bit::rrc::RRC_A::new()),
                        "(hl)" => Box::new(bit::rrc::rrc_phl::RRC_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "rl" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::rl::rl_pixd::RL_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::rl::rl_piyd::RL_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::rl::RL_B::new()),
                        "c" => Box::new(bit::rl::RL_C::new()),
                        "d" => Box::new(bit::rl::RL_D::new()),
                        "e" => Box::new(bit::rl::RL_E::new()),
                        "h" => Box::new(bit::rl::RL_H::new()),
                        "l" => Box::new(bit::rl::RL_L::new()),
                        "a" => Box::new(bit::rl::RL_A::new()),
                        "(hl)" => Box::new(bit::rl::rl_phl::RL_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "sra" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::sra::sra_pixd::SRA_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::sra::sra_piyd::SRA_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::sra::SRA_B::new()),
                        "c" => Box::new(bit::sra::SRA_C::new()),
                        "d" => Box::new(bit::sra::SRA_D::new()),
                        "e" => Box::new(bit::sra::SRA_E::new()),
                        "h" => Box::new(bit::sra::SRA_H::new()),
                        "l" => Box::new(bit::sra::SRA_L::new()),
                        "a" => Box::new(bit::sra::SRA_A::new()),
                        "(hl)" => Box::new(bit::sra::sra_phl::SRA_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "srl" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::srl::srl_pixd::SRL_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::srl::srl_piyd::SRL_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::srl::SRL_B::new()),
                        "c" => Box::new(bit::srl::SRL_C::new()),
                        "d" => Box::new(bit::srl::SRL_D::new()),
                        "e" => Box::new(bit::srl::SRL_E::new()),
                        "h" => Box::new(bit::srl::SRL_H::new()),
                        "l" => Box::new(bit::srl::SRL_L::new()),
                        "a" => Box::new(bit::srl::SRL_A::new()),
                        "(hl)" => Box::new(bit::srl::srl_phl::SRL_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "sll" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::sll::sll_pixd::SLL_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::sll::sll_piyd::SLL_PIYD::new_with_value(offset))
                    }
                    _ => match destination {
                        "b" => Box::new(bit::sll::SLL_B::new()),
                        "c" => Box::new(bit::sll::SLL_C::new()),
                        "d" => Box::new(bit::sll::SLL_D::new()),
                        "e" => Box::new(bit::sll::SLL_E::new()),
                        "h" => Box::new(bit::sll::SLL_H::new()),
                        "l" => Box::new(bit::sll::SLL_L::new()),
                        "a" => Box::new(bit::sll::SLL_A::new()),
                        "(hl)" => Box::new(bit::sll::sll_phl::SLL_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                }
            }
            "sla" => {
                let destination = get_op(2)?;
                match is_val(destination) {
                    Ok(ImmediateValue::OffsetIX(offset)) => {
                        Box::new(bit::sla::sla_pixd::SLA_PIXD::new_with_value(offset))
                    }
                    Ok(ImmediateValue::OffsetIY(offset)) => {
                        Box::new(bit::sla::sla_piyd::SLA_PIYD::new_with_value(offset))
                    }
                    Err(_) => match destination {
                        "b" => Box::new(bit::sla::SLA_B::new()),
                        "c" => Box::new(bit::sla::SLA_C::new()),
                        "d" => Box::new(bit::sla::SLA_D::new()),
                        "e" => Box::new(bit::sla::SLA_E::new()),
                        "h" => Box::new(bit::sla::SLA_H::new()),
                        "l" => Box::new(bit::sla::SLA_L::new()),
                        "a" => Box::new(bit::sla::SLA_A::new()),
                        "(hl)" => Box::new(bit::sla::sla_phl::SLA_PHL::new()),
                        _ => {
                            return Err(ParseError::InvalidInstruction(
                                "Invalid instruction".to_string(),
                            ))
                        }
                    },
                    _ => {
                        return Err(ParseError::InvalidInstruction(
                            "Invalid instruction".to_string(),
                        ))
                    }
                }
            }
            "out" => {
                let port = get_op(2)?;
                let register = get_op(3)?;
                match (port, register, is_val(port)) {
                    (_, "a", Ok(ImmediateValue::Val8(val))) => {
                        Box::new(io::out_n_a::OUT_N_A::new_with_value(val))
                    }
                    ("(c)", "b", _) => Box::new(io::out_c_b::OUT_C_B::new()),
                    ("(c)", "c", _) => Box::new(io::out_c_c::OUT_C_C::new()),
                    ("(c)", "d", _) => Box::new(io::out_c_d::OUT_C_D::new()),
                    ("(c)", "e", _) => Box::new(io::out_c_e::OUT_C_E::new()),
                    ("(c)", "h", _) => Box::new(io::out_c_h::OUT_C_H::new()),
                    ("(c)", "l", _) => Box::new(io::out_c_l::OUT_C_L::new()),
                    ("(c)", "a", _) => Box::new(io::out_c_a::OUT_C_A::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(
                            "Invalid instruction".to_string(),
                        ))
                    }
                }
            }
            "in" => {
                let register = get_op(2)?;
                let port = get_op(3)?;
                match (register, port, is_val(port)) {
                    ("a", _, Ok(ImmediateValue::Val8(val))) => {
                        Box::new(io::in_a_n::IN_A_N::new_with_value(val))
                    }
                    ("b", "(c)", _) => Box::new(io::in_b_c::IN_B_C::new()),
                    ("c", "(c)", _) => Box::new(io::in_c_c::IN_C_C::new()),
                    ("d", "(c)", _) => Box::new(io::in_d_c::IN_D_C::new()),
                    ("e", "(c)", _) => Box::new(io::in_e_c::IN_E_C::new()),
                    ("h", "(c)", _) => Box::new(io::in_h_c::IN_H_C::new()),
                    ("l", "(c)", _) => Box::new(io::in_l_c::IN_L_C::new()),
                    ("a", "(c)", _) => Box::new(io::in_a_c::IN_A_C::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(
                            "Invalid instruction".to_string(),
                        ))
                    }
                }
            }
            "daa" => Box::new(daa::DAA::new()),
            "cpl" => Box::new(cpl::CPL::new()),
            _ => {
                return Err(ParseError::InvalidInstruction(
                    "Invalid instruction".to_string(),
                ))
            }
        };
        Ok(instruction)
    }

    fn ins_from_machinecode(
        &self,
        memory: &dyn MemoryDevice,
        pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<Z80>)>, ParseError> {
        let ins_byte0 = memory.read_8(pos)?;
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match ins_byte0 {
            0x00u8 => Box::new(nop::NOP::new()),
            0x01 => Box::new(ld::LD_BC_NN::new(memory, pos)?),
            0x02 => Box::new(ld::LD_PBC_A::new()),
            0x03 => Box::new(math::inc::INC_BC::new()),
            0x04 => Box::new(math::inc::INC_B::new()),
            0x05 => Box::new(math::dec::DEC_B::new()),
            0x06 => Box::new(ld::LD_B_N::new(memory, pos)?),
            0x07 => Box::new(rlca::RLCA::new()),
            0x08 => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
            0x09 => Box::new(math::add::ADD_HL_BC::new()),
            0x0A => Box::new(ld::LD_A_PBC::new()),
            0x0B => Box::new(math::dec::DEC_BC::new()),
            0x0C => Box::new(math::inc::INC_C::new()),
            0x0D => Box::new(math::dec::DEC_C::new()),
            0x0E => Box::new(ld::LD_C_N::new(memory, pos)?),
            0x0F => Box::new(rrca::RRCA::new()),
            0x10 => Box::new(djnz_d::DJNZ_D::new(memory, pos)?),
            0x11 => Box::new(ld::LD_DE_NN::new(memory, pos)?),
            0x12 => Box::new(ld::LD_PDE_A::new()),
            0x13 => Box::new(math::inc::INC_DE::new()),
            0x14 => Box::new(math::inc::INC_D::new()),
            0x15 => Box::new(math::dec::DEC_D::new()),
            0x16 => Box::new(ld::LD_D_N::new(memory, pos)?),
            0x17 => Box::new(rla::RLA::new()),
            0x18 => Box::new(jump::jr::jr_d::JR_D::new(memory, pos)?),
            0x19 => Box::new(math::add::ADD_HL_DE::new()),
            0x1A => Box::new(ld::LD_A_PDE::new()),
            0x1B => Box::new(math::dec::DEC_DE::new()),
            0x1C => Box::new(math::inc::INC_E::new()),
            0x1D => Box::new(math::dec::DEC_E::new()),
            0x1E => Box::new(ld::LD_E_N::new(memory, pos)?),
            0x1F => Box::new(rra::RRA::new()),
            0x20 => Box::new(jump::jr::jr_nz_d::JR_NZ_D::new(memory, pos)?),
            0x21 => Box::new(ld::LD_HL_NN::new(memory, pos)?),
            0x22 => Box::new(ld::LD_PNN_HL::new(memory, pos)?),
            0x23 => Box::new(math::inc::INC_HL::new()),
            0x24 => Box::new(math::inc::INC_H::new()),
            0x25 => Box::new(math::dec::DEC_H::new()),
            0x26 => Box::new(ld::LD_H_N::new(memory, pos)?),
            0x27 => Box::new(daa::DAA::new()),
            0x28 => Box::new(jump::jr::jr_z_d::JR_Z_D::new(memory, pos)?),
            0x29 => Box::new(math::add::ADD_HL_HL::new()),
            0x2A => Box::new(ld::LD_HL_PNN::new(memory, pos)?),
            0x2B => Box::new(math::dec::DEC_HL::new()),
            0x2C => Box::new(math::inc::INC_L::new()),
            0x2D => Box::new(math::dec::DEC_L::new()),
            0x2E => Box::new(ld::LD_L_N::new(memory, pos)?),
            0x2F => Box::new(cpl::CPL::new()),
            0x30 => Box::new(jump::jr::jr_nc_d::JR_NC_D::new(memory, pos)?),
            0x31 => Box::new(ld::ld_sp_nn::LD_SP_NN::new(memory, pos)?),
            0x32 => Box::new(ld::LD_PNN_A::new(memory, pos)?),
            0x33 => Box::new(math::inc::inc_sp::INC_SP::new()),
            0x34 => Box::new(math::inc::inc_phl::INC_PHL::new()),
            0x35 => Box::new(math::dec::dec_phl::DEC_PHL::new()),
            0x36 => Box::new(ld::LD_PHL_N::new(memory, pos)?),
            0x37 => Box::new(scf::SCF::new()),
            0x38 => Box::new(jump::jr::jr_c_d::JR_C_D::new(memory, pos)?),
            0x39 => Box::new(math::add::add_hl_sp::ADD_HL_SP::new()),
            0x3A => Box::new(ld::LD_A_PNN::new(memory, pos)?),
            0x3B => Box::new(math::dec::dec_sp::DEC_SP::new()),
            0x3C => Box::new(math::inc::INC_A::new()),
            0x3D => Box::new(math::dec::DEC_A::new()),
            0x3E => Box::new(ld::LD_A_N::new(memory, pos)?),
            0x3F => Box::new(ccf::CCF::new()),
            0x40 => Box::new(ld::LD_B_B::new()),
            0x41 => Box::new(ld::LD_B_C::new()),
            0x42 => Box::new(ld::LD_B_D::new()),
            0x43 => Box::new(ld::LD_B_E::new()),
            0x44 => Box::new(ld::LD_B_H::new()),
            0x45 => Box::new(ld::LD_B_L::new()),
            0x46 => Box::new(ld::LD_B_PHL::new()),
            0x47 => Box::new(ld::LD_B_A::new()),
            0x48 => Box::new(ld::LD_C_B::new()),
            0x49 => Box::new(ld::LD_C_C::new()),
            0x4A => Box::new(ld::LD_C_D::new()),
            0x4B => Box::new(ld::LD_C_E::new()),
            0x4C => Box::new(ld::LD_C_H::new()),
            0x4D => Box::new(ld::LD_C_L::new()),
            0x4E => Box::new(ld::LD_C_PHL::new()),
            0x4F => Box::new(ld::LD_C_A::new()),
            0x50 => Box::new(ld::LD_D_B::new()),
            0x51 => Box::new(ld::LD_D_C::new()),
            0x52 => Box::new(ld::LD_D_D::new()),
            0x53 => Box::new(ld::LD_D_E::new()),
            0x54 => Box::new(ld::LD_D_H::new()),
            0x55 => Box::new(ld::LD_D_L::new()),
            0x56 => Box::new(ld::LD_D_PHL::new()),
            0x57 => Box::new(ld::LD_D_A::new()),
            0x58 => Box::new(ld::LD_E_B::new()),
            0x59 => Box::new(ld::LD_E_C::new()),
            0x5A => Box::new(ld::LD_E_D::new()),
            0x5B => Box::new(ld::LD_E_E::new()),
            0x5C => Box::new(ld::LD_E_H::new()),
            0x5D => Box::new(ld::LD_E_L::new()),
            0x5E => Box::new(ld::LD_E_PHL::new()),
            0x5F => Box::new(ld::LD_E_A::new()),
            0x60 => Box::new(ld::LD_H_B::new()),
            0x61 => Box::new(ld::LD_H_C::new()),
            0x62 => Box::new(ld::LD_H_D::new()),
            0x63 => Box::new(ld::LD_H_E::new()),
            0x64 => Box::new(ld::LD_H_H::new()),
            0x65 => Box::new(ld::LD_H_L::new()),
            0x66 => Box::new(ld::LD_H_PHL::new()),
            0x67 => Box::new(ld::LD_H_A::new()),
            0x68 => Box::new(ld::LD_L_B::new()),
            0x69 => Box::new(ld::LD_L_C::new()),
            0x6A => Box::new(ld::LD_L_D::new()),
            0x6B => Box::new(ld::LD_L_E::new()),
            0x6C => Box::new(ld::LD_L_H::new()),
            0x6D => Box::new(ld::LD_L_L::new()),
            0x6E => Box::new(ld::LD_L_PHL::new()),
            0x6F => Box::new(ld::LD_L_A::new()),
            0x70 => Box::new(ld::LD_PHL_B::new()),
            0x71 => Box::new(ld::LD_PHL_C::new()),
            0x72 => Box::new(ld::LD_PHL_D::new()),
            0x73 => Box::new(ld::LD_PHL_E::new()),
            0x74 => Box::new(ld::LD_PHL_H::new()),
            0x75 => Box::new(ld::LD_PHL_L::new()),
            0x76 => Box::new(halt::Halt::new()),
            0x77 => Box::new(ld::LD_PHL_A::new()),
            0x78 => Box::new(ld::LD_A_B::new()),
            0x79 => Box::new(ld::LD_A_C::new()),
            0x7A => Box::new(ld::LD_A_D::new()),
            0x7B => Box::new(ld::LD_A_E::new()),
            0x7C => Box::new(ld::LD_A_H::new()),
            0x7D => Box::new(ld::LD_A_L::new()),
            0x7E => Box::new(ld::LD_A_PHL::new()),
            0x7F => Box::new(ld::LD_A_A::new()),
            0x80 => Box::new(math::add::ADD_A_B::new()),
            0x81 => Box::new(math::add::ADD_A_C::new()),
            0x82 => Box::new(math::add::ADD_A_D::new()),
            0x83 => Box::new(math::add::ADD_A_E::new()),
            0x84 => Box::new(math::add::ADD_A_H::new()),
            0x85 => Box::new(math::add::ADD_A_L::new()),
            0x86 => Box::new(math::add::add_a_phl::ADD_A_PHL::new()),
            0x87 => Box::new(math::add::ADD_A_A::new()),
            0x88 => Box::new(math::adc::ADC_A_B::new()),
            0x89 => Box::new(math::adc::ADC_A_C::new()),
            0x8A => Box::new(math::adc::ADC_A_D::new()),
            0x8B => Box::new(math::adc::ADC_A_E::new()),
            0x8C => Box::new(math::adc::ADC_A_H::new()),
            0x8D => Box::new(math::adc::ADC_A_L::new()),
            0x8E => Box::new(math::adc::adc_a_phl::ADC_A_PHL::new()),
            0x8F => Box::new(math::adc::ADC_A_A::new()),
            0x90 => Box::new(math::sub::SUB_B::new()),
            0x91 => Box::new(math::sub::SUB_C::new()),
            0x92 => Box::new(math::sub::SUB_D::new()),
            0x93 => Box::new(math::sub::SUB_E::new()),
            0x94 => Box::new(math::sub::SUB_H::new()),
            0x95 => Box::new(math::sub::SUB_L::new()),
            0x96 => Box::new(math::sub::sub_phl::SUB_PHL::new()),
            0x97 => Box::new(math::sub::SUB_A::new()),
            0x98 => Box::new(math::sbc::SBC_A_B::new()),
            0x99 => Box::new(math::sbc::SBC_A_C::new()),
            0x9A => Box::new(math::sbc::SBC_A_D::new()),
            0x9B => Box::new(math::sbc::SBC_A_E::new()),
            0x9C => Box::new(math::sbc::SBC_A_H::new()),
            0x9D => Box::new(math::sbc::SBC_A_L::new()),
            0x9E => Box::new(math::sbc::sbc_a_phl::SBC_A_PHL::new()),
            0x9F => Box::new(math::sbc::SBC_A_A::new()),
            0xA0 => Box::new(math::and::AND_B::new()),
            0xA1 => Box::new(math::and::AND_C::new()),
            0xA2 => Box::new(math::and::AND_D::new()),
            0xA3 => Box::new(math::and::AND_E::new()),
            0xA4 => Box::new(math::and::AND_H::new()),
            0xA5 => Box::new(math::and::AND_L::new()),
            0xA6 => Box::new(math::and::and_phl::AND_PHL::new()),
            0xA7 => Box::new(math::and::AND_A::new()),
            0xA8 => Box::new(math::xor::XOR_B::new()),
            0xA9 => Box::new(math::xor::XOR_C::new()),
            0xAA => Box::new(math::xor::XOR_D::new()),
            0xAB => Box::new(math::xor::XOR_E::new()),
            0xAC => Box::new(math::xor::XOR_H::new()),
            0xAD => Box::new(math::xor::XOR_L::new()),
            0xAE => Box::new(math::xor::xor_phl::XOR_PHL::new()),
            0xAF => Box::new(math::xor::XOR_A::new()),
            0xB0 => Box::new(math::or::OR_B::new()),
            0xB1 => Box::new(math::or::OR_C::new()),
            0xB2 => Box::new(math::or::OR_D::new()),
            0xB3 => Box::new(math::or::OR_E::new()),
            0xB4 => Box::new(math::or::OR_H::new()),
            0xB5 => Box::new(math::or::OR_L::new()),
            0xB6 => Box::new(math::or::or_phl::OR_PHL::new()),
            0xB7 => Box::new(math::or::OR_A::new()),
            0xB8 => Box::new(math::cp::CP_B::new()),
            0xB9 => Box::new(math::cp::CP_C::new()),
            0xBA => Box::new(math::cp::CP_D::new()),
            0xBB => Box::new(math::cp::CP_E::new()),
            0xBC => Box::new(math::cp::CP_H::new()),
            0xBD => Box::new(math::cp::CP_L::new()),
            0xBE => Box::new(math::cp::cp_phl::CP_PHL::new()),
            0xBF => Box::new(math::cp::CP_A::new()),
            0xC0 => Box::new(ret::ret_nz::RET_NZ::new()),
            0xC1 => Box::new(stack::pop::POP_BC::new()),
            0xC2 => Box::new(jump::jp::jp_nz_nn::JP_NZ_NN::new(memory, pos)?),
            0xC3 => Box::new(jump::jp::jp_nn::JP_NN::new(memory, pos)?),
            0xC4 => Box::new(call::call_nz_nn::CALL_NZ_NN::new(memory, pos)?),
            0xC5 => Box::new(stack::push::PUSH_BC::new()),
            0xC6 => Box::new(math::add::add_a_n::ADD_A_N::new(memory, pos)?),
            0xC7 => Box::new(rst::RST_0x00::new()),
            0xC8 => Box::new(ret::ret_z::RET_Z::new()),
            0xC9 => Box::new(ret::ret::RET::new()),
            0xCA => Box::new(jump::jp::jp_z_nn::JP_Z_NN::new(memory, pos)?),
            0xCB => match memory.read_8(pos.wrapping_add(1))? {
                0x00 => Box::new(bit::rlc::RLC_B::new()),
                0x01 => Box::new(bit::rlc::RLC_C::new()),
                0x02 => Box::new(bit::rlc::RLC_D::new()),
                0x03 => Box::new(bit::rlc::RLC_E::new()),
                0x04 => Box::new(bit::rlc::RLC_H::new()),
                0x05 => Box::new(bit::rlc::RLC_L::new()),
                0x06 => Box::new(bit::rlc::rlc_phl::RLC_PHL::new()),
                0x07 => Box::new(bit::rlc::RLC_A::new()),
                0x08 => Box::new(bit::rrc::RRC_B::new()),
                0x09 => Box::new(bit::rrc::RRC_C::new()),
                0x0A => Box::new(bit::rrc::RRC_D::new()),
                0x0B => Box::new(bit::rrc::RRC_E::new()),
                0x0C => Box::new(bit::rrc::RRC_H::new()),
                0x0D => Box::new(bit::rrc::RRC_L::new()),
                0x0E => Box::new(bit::rrc::rrc_phl::RRC_PHL::new()),
                0x0F => Box::new(bit::rrc::RRC_A::new()),
                0x10 => Box::new(bit::rl::RL_B::new()),
                0x11 => Box::new(bit::rl::RL_C::new()),
                0x12 => Box::new(bit::rl::RL_D::new()),
                0x13 => Box::new(bit::rl::RL_E::new()),
                0x14 => Box::new(bit::rl::RL_H::new()),
                0x15 => Box::new(bit::rl::RL_L::new()),
                0x16 => Box::new(bit::rl::rl_phl::RL_PHL::new()),
                0x17 => Box::new(bit::rl::RL_A::new()),
                0x18 => Box::new(bit::rr::RR_B::new()),
                0x19 => Box::new(bit::rr::RR_C::new()),
                0x1A => Box::new(bit::rr::RR_D::new()),
                0x1B => Box::new(bit::rr::RR_E::new()),
                0x1C => Box::new(bit::rr::RR_H::new()),
                0x1D => Box::new(bit::rr::RR_L::new()),
                0x1E => Box::new(bit::rr::rr_phl::RR_PHL::new()),
                0x1F => Box::new(bit::rr::RR_A::new()),
                0x20 => Box::new(bit::sla::SLA_B::new()),
                0x21 => Box::new(bit::sla::SLA_C::new()),
                0x22 => Box::new(bit::sla::SLA_D::new()),
                0x23 => Box::new(bit::sla::SLA_E::new()),
                0x24 => Box::new(bit::sla::SLA_H::new()),
                0x25 => Box::new(bit::sla::SLA_L::new()),
                0x26 => Box::new(bit::sla::sla_phl::SLA_PHL::new()),
                0x27 => Box::new(bit::sla::SLA_A::new()),
                0x28 => Box::new(bit::sra::SRA_B::new()),
                0x29 => Box::new(bit::sra::SRA_C::new()),
                0x2A => Box::new(bit::sra::SRA_D::new()),
                0x2B => Box::new(bit::sra::SRA_E::new()),
                0x2C => Box::new(bit::sra::SRA_H::new()),
                0x2D => Box::new(bit::sra::SRA_L::new()),
                0x2E => Box::new(bit::sra::sra_phl::SRA_PHL::new()),
                0x2F => Box::new(bit::sra::SRA_A::new()),
                0x30 => Box::new(bit::sll::SLL_B::new()),
                0x31 => Box::new(bit::sll::SLL_C::new()),
                0x32 => Box::new(bit::sll::SLL_D::new()),
                0x33 => Box::new(bit::sll::SLL_E::new()),
                0x34 => Box::new(bit::sll::SLL_H::new()),
                0x35 => Box::new(bit::sll::SLL_L::new()),
                0x36 => Box::new(bit::sll::sll_phl::SLL_PHL::new()),
                0x37 => Box::new(bit::sll::SLL_A::new()),
                0x38 => Box::new(bit::srl::SRL_B::new()),
                0x39 => Box::new(bit::srl::SRL_C::new()),
                0x3A => Box::new(bit::srl::SRL_D::new()),
                0x3B => Box::new(bit::srl::SRL_E::new()),
                0x3C => Box::new(bit::srl::SRL_H::new()),
                0x3D => Box::new(bit::srl::SRL_L::new()),
                0x3E => Box::new(bit::srl::srl_phl::SRL_PHL::new()),
                0x3F => Box::new(bit::srl::SRL_A::new()),
                0x40 => Box::new(bit::bit::BIT_0_B::new()),
                0x41 => Box::new(bit::bit::BIT_0_C::new()),
                0x42 => Box::new(bit::bit::BIT_0_D::new()),
                0x43 => Box::new(bit::bit::BIT_0_E::new()),
                0x44 => Box::new(bit::bit::BIT_0_H::new()),
                0x45 => Box::new(bit::bit::BIT_0_L::new()),
                0x46 => Box::new(bit::bit::BIT_0_PHL::new()),
                0x47 => Box::new(bit::bit::BIT_0_A::new()),
                0x48 => Box::new(bit::bit::BIT_1_B::new()),
                0x49 => Box::new(bit::bit::BIT_1_C::new()),
                0x4A => Box::new(bit::bit::BIT_1_D::new()),
                0x4B => Box::new(bit::bit::BIT_1_E::new()),
                0x4C => Box::new(bit::bit::BIT_1_H::new()),
                0x4D => Box::new(bit::bit::BIT_1_L::new()),
                0x4E => Box::new(bit::bit::BIT_1_PHL::new()),
                0x4F => Box::new(bit::bit::BIT_1_A::new()),
                0x50 => Box::new(bit::bit::BIT_2_B::new()),
                0x51 => Box::new(bit::bit::BIT_2_C::new()),
                0x52 => Box::new(bit::bit::BIT_2_D::new()),
                0x53 => Box::new(bit::bit::BIT_2_E::new()),
                0x54 => Box::new(bit::bit::BIT_2_H::new()),
                0x55 => Box::new(bit::bit::BIT_2_L::new()),
                0x56 => Box::new(bit::bit::BIT_2_PHL::new()),
                0x57 => Box::new(bit::bit::BIT_2_A::new()),
                0x58 => Box::new(bit::bit::BIT_3_B::new()),
                0x59 => Box::new(bit::bit::BIT_3_C::new()),
                0x5A => Box::new(bit::bit::BIT_3_D::new()),
                0x5B => Box::new(bit::bit::BIT_3_E::new()),
                0x5C => Box::new(bit::bit::BIT_3_H::new()),
                0x5D => Box::new(bit::bit::BIT_3_L::new()),
                0x5E => Box::new(bit::bit::BIT_3_PHL::new()),
                0x5F => Box::new(bit::bit::BIT_3_A::new()),
                0x60 => Box::new(bit::bit::BIT_4_B::new()),
                0x61 => Box::new(bit::bit::BIT_4_C::new()),
                0x62 => Box::new(bit::bit::BIT_4_D::new()),
                0x63 => Box::new(bit::bit::BIT_4_E::new()),
                0x64 => Box::new(bit::bit::BIT_4_H::new()),
                0x65 => Box::new(bit::bit::BIT_4_L::new()),
                0x66 => Box::new(bit::bit::BIT_4_PHL::new()),
                0x67 => Box::new(bit::bit::BIT_4_A::new()),
                0x68 => Box::new(bit::bit::BIT_5_B::new()),
                0x69 => Box::new(bit::bit::BIT_5_C::new()),
                0x6A => Box::new(bit::bit::BIT_5_D::new()),
                0x6B => Box::new(bit::bit::BIT_5_E::new()),
                0x6C => Box::new(bit::bit::BIT_5_H::new()),
                0x6D => Box::new(bit::bit::BIT_5_L::new()),
                0x6E => Box::new(bit::bit::BIT_5_PHL::new()),
                0x6F => Box::new(bit::bit::BIT_5_A::new()),
                0x70 => Box::new(bit::bit::BIT_6_B::new()),
                0x71 => Box::new(bit::bit::BIT_6_C::new()),
                0x72 => Box::new(bit::bit::BIT_6_D::new()),
                0x73 => Box::new(bit::bit::BIT_6_E::new()),
                0x74 => Box::new(bit::bit::BIT_6_H::new()),
                0x75 => Box::new(bit::bit::BIT_6_L::new()),
                0x76 => Box::new(bit::bit::BIT_6_PHL::new()),
                0x77 => Box::new(bit::bit::BIT_6_A::new()),
                0x78 => Box::new(bit::bit::BIT_7_B::new()),
                0x79 => Box::new(bit::bit::BIT_7_C::new()),
                0x7A => Box::new(bit::bit::BIT_7_D::new()),
                0x7B => Box::new(bit::bit::BIT_7_E::new()),
                0x7C => Box::new(bit::bit::BIT_7_H::new()),
                0x7D => Box::new(bit::bit::BIT_7_L::new()),
                0x7E => Box::new(bit::bit::BIT_7_PHL::new()),
                0x7F => Box::new(bit::bit::BIT_7_A::new()),
                0x80 => Box::new(bit::res::RES_0_B::new()),
                0x81 => Box::new(bit::res::RES_0_C::new()),
                0x82 => Box::new(bit::res::RES_0_D::new()),
                0x83 => Box::new(bit::res::RES_0_E::new()),
                0x84 => Box::new(bit::res::RES_0_H::new()),
                0x85 => Box::new(bit::res::RES_0_L::new()),
                0x86 => Box::new(bit::res::RES_0_PHL::new()),
                0x87 => Box::new(bit::res::RES_0_A::new()),
                0x88 => Box::new(bit::res::RES_1_B::new()),
                0x89 => Box::new(bit::res::RES_1_C::new()),
                0x8A => Box::new(bit::res::RES_1_D::new()),
                0x8B => Box::new(bit::res::RES_1_E::new()),
                0x8C => Box::new(bit::res::RES_1_H::new()),
                0x8D => Box::new(bit::res::RES_1_L::new()),
                0x8E => Box::new(bit::res::RES_1_PHL::new()),
                0x8F => Box::new(bit::res::RES_1_A::new()),
                0x90 => Box::new(bit::res::RES_2_B::new()),
                0x91 => Box::new(bit::res::RES_2_C::new()),
                0x92 => Box::new(bit::res::RES_2_D::new()),
                0x93 => Box::new(bit::res::RES_2_E::new()),
                0x94 => Box::new(bit::res::RES_2_H::new()),
                0x95 => Box::new(bit::res::RES_2_L::new()),
                0x96 => Box::new(bit::res::RES_2_PHL::new()),
                0x97 => Box::new(bit::res::RES_2_A::new()),
                0x98 => Box::new(bit::res::RES_3_B::new()),
                0x99 => Box::new(bit::res::RES_3_C::new()),
                0x9A => Box::new(bit::res::RES_3_D::new()),
                0x9B => Box::new(bit::res::RES_3_E::new()),
                0x9C => Box::new(bit::res::RES_3_H::new()),
                0x9D => Box::new(bit::res::RES_3_L::new()),
                0x9E => Box::new(bit::res::RES_3_PHL::new()),
                0x9F => Box::new(bit::res::RES_3_A::new()),
                0xA0 => Box::new(bit::res::RES_4_B::new()),
                0xA1 => Box::new(bit::res::RES_4_C::new()),
                0xA2 => Box::new(bit::res::RES_4_D::new()),
                0xA3 => Box::new(bit::res::RES_4_E::new()),
                0xA4 => Box::new(bit::res::RES_4_H::new()),
                0xA5 => Box::new(bit::res::RES_4_L::new()),
                0xA6 => Box::new(bit::res::RES_4_PHL::new()),
                0xA7 => Box::new(bit::res::RES_4_A::new()),
                0xA8 => Box::new(bit::res::RES_5_B::new()),
                0xA9 => Box::new(bit::res::RES_5_C::new()),
                0xAA => Box::new(bit::res::RES_5_D::new()),
                0xAB => Box::new(bit::res::RES_5_E::new()),
                0xAC => Box::new(bit::res::RES_5_H::new()),
                0xAD => Box::new(bit::res::RES_5_L::new()),
                0xAE => Box::new(bit::res::RES_5_PHL::new()),
                0xAF => Box::new(bit::res::RES_5_A::new()),
                0xB0 => Box::new(bit::res::RES_6_B::new()),
                0xB1 => Box::new(bit::res::RES_6_C::new()),
                0xB2 => Box::new(bit::res::RES_6_D::new()),
                0xB3 => Box::new(bit::res::RES_6_E::new()),
                0xB4 => Box::new(bit::res::RES_6_H::new()),
                0xB5 => Box::new(bit::res::RES_6_L::new()),
                0xB6 => Box::new(bit::res::RES_6_PHL::new()),
                0xB7 => Box::new(bit::res::RES_6_A::new()),
                0xB8 => Box::new(bit::res::RES_7_B::new()),
                0xB9 => Box::new(bit::res::RES_7_C::new()),
                0xBA => Box::new(bit::res::RES_7_D::new()),
                0xBB => Box::new(bit::res::RES_7_E::new()),
                0xBC => Box::new(bit::res::RES_7_H::new()),
                0xBD => Box::new(bit::res::RES_7_L::new()),
                0xBE => Box::new(bit::res::RES_7_PHL::new()),
                0xBF => Box::new(bit::res::RES_7_A::new()),
                0xC0 => Box::new(bit::set::SET_0_B::new()),
                0xC1 => Box::new(bit::set::SET_0_C::new()),
                0xC2 => Box::new(bit::set::SET_0_D::new()),
                0xC3 => Box::new(bit::set::SET_0_E::new()),
                0xC4 => Box::new(bit::set::SET_0_H::new()),
                0xC5 => Box::new(bit::set::SET_0_L::new()),
                0xC6 => Box::new(bit::set::SET_0_PHL::new()),
                0xC7 => Box::new(bit::set::SET_0_A::new()),
                0xC8 => Box::new(bit::set::SET_1_B::new()),
                0xC9 => Box::new(bit::set::SET_1_C::new()),
                0xCA => Box::new(bit::set::SET_1_D::new()),
                0xCB => Box::new(bit::set::SET_1_E::new()),
                0xCC => Box::new(bit::set::SET_1_H::new()),
                0xCD => Box::new(bit::set::SET_1_L::new()),
                0xCE => Box::new(bit::set::SET_1_PHL::new()),
                0xCF => Box::new(bit::set::SET_1_A::new()),
                0xD0 => Box::new(bit::set::SET_2_B::new()),
                0xD1 => Box::new(bit::set::SET_2_C::new()),
                0xD2 => Box::new(bit::set::SET_2_D::new()),
                0xD3 => Box::new(bit::set::SET_2_E::new()),
                0xD4 => Box::new(bit::set::SET_2_H::new()),
                0xD5 => Box::new(bit::set::SET_2_L::new()),
                0xD6 => Box::new(bit::set::SET_2_PHL::new()),
                0xD7 => Box::new(bit::set::SET_2_A::new()),
                0xD8 => Box::new(bit::set::SET_3_B::new()),
                0xD9 => Box::new(bit::set::SET_3_C::new()),
                0xDA => Box::new(bit::set::SET_3_D::new()),
                0xDB => Box::new(bit::set::SET_3_E::new()),
                0xDC => Box::new(bit::set::SET_3_H::new()),
                0xDD => Box::new(bit::set::SET_3_L::new()),
                0xDE => Box::new(bit::set::SET_3_PHL::new()),
                0xDF => Box::new(bit::set::SET_3_A::new()),
                0xE0 => Box::new(bit::set::SET_4_B::new()),
                0xE1 => Box::new(bit::set::SET_4_C::new()),
                0xE2 => Box::new(bit::set::SET_4_D::new()),
                0xE3 => Box::new(bit::set::SET_4_E::new()),
                0xE4 => Box::new(bit::set::SET_4_H::new()),
                0xE5 => Box::new(bit::set::SET_4_L::new()),
                0xE6 => Box::new(bit::set::SET_4_PHL::new()),
                0xE7 => Box::new(bit::set::SET_4_A::new()),
                0xE8 => Box::new(bit::set::SET_5_B::new()),
                0xE9 => Box::new(bit::set::SET_5_C::new()),
                0xEA => Box::new(bit::set::SET_5_D::new()),
                0xEB => Box::new(bit::set::SET_5_E::new()),
                0xEC => Box::new(bit::set::SET_5_H::new()),
                0xED => Box::new(bit::set::SET_5_L::new()),
                0xEE => Box::new(bit::set::SET_5_PHL::new()),
                0xEF => Box::new(bit::set::SET_5_A::new()),
                0xF0 => Box::new(bit::set::SET_6_B::new()),
                0xF1 => Box::new(bit::set::SET_6_C::new()),
                0xF2 => Box::new(bit::set::SET_6_D::new()),
                0xF3 => Box::new(bit::set::SET_6_E::new()),
                0xF4 => Box::new(bit::set::SET_6_H::new()),
                0xF5 => Box::new(bit::set::SET_6_L::new()),
                0xF6 => Box::new(bit::set::SET_6_PHL::new()),
                0xF7 => Box::new(bit::set::SET_6_A::new()),
                0xF8 => Box::new(bit::set::SET_7_B::new()),
                0xF9 => Box::new(bit::set::SET_7_C::new()),
                0xFA => Box::new(bit::set::SET_7_D::new()),
                0xFB => Box::new(bit::set::SET_7_E::new()),
                0xFC => Box::new(bit::set::SET_7_H::new()),
                0xFD => Box::new(bit::set::SET_7_L::new()),
                0xFE => Box::new(bit::set::SET_7_PHL::new()),
                0xFF => Box::new(bit::set::SET_7_A::new()),
            },
            0xCC => Box::new(call::call_z_nn::CALL_Z_NN::new(memory, pos)?),
            0xCD => Box::new(call::call_nn::CALL_NN::new(memory, pos)?),
            0xCE => Box::new(math::adc::adc_a_n::ADC_A_N::new(memory, pos)?),
            0xCF => Box::new(rst::RST_0x08::new()),
            0xD0 => Box::new(ret::ret_nc::RET_NC::new()),
            0xD1 => Box::new(stack::pop::POP_DE::new()),
            0xD2 => Box::new(jump::jp::jp_nc_nn::JP_NC_NN::new(memory, pos)?),
            0xD3 => Box::new(io::out_n_a::OUT_N_A::new(memory, pos)?),
            0xD4 => Box::new(call::call_nc_nn::CALL_NC_NN::new(memory, pos)?),
            0xD5 => Box::new(stack::push::PUSH_DE::new()),
            0xD6 => Box::new(math::sub::sub_n::SUB_N::new(memory, pos)?),
            0xD7 => Box::new(rst::RST_0x10::new()),
            0xD8 => Box::new(ret::ret_c::RET_C::new()),
            0xD9 => Box::new(exx::EXX::new()),
            0xDA => Box::new(jump::jp::jp_c_nn::JP_C_NN::new(memory, pos)?),
            0xDB => Box::new(io::in_a_n::IN_A_N::new(memory, pos)?),
            0xDC => Box::new(call::call_c_nn::CALL_C_NN::new(memory, pos)?),
            0xDD => {
                let ins_byte1 = memory.read_8(pos.wrapping_add(1))?;
                match ins_byte1 {
                    0x09 => Box::new(math::add::add_ix_bc::ADD_IX_BC::new()),
                    0x19 => Box::new(math::add::add_ix_de::ADD_IX_DE::new()),
                    0x21 => Box::new(ld::ld_ix_nn::LD_IX_NN::new(memory, pos)?),
                    0x22 => Box::new(ld::ld_pnn_ix::LD_PNN_IX::new(memory, pos)?),
                    0x23 => Box::new(math::inc::inc_ix::INC_IX::new()),
                    0x29 => Box::new(math::add::add_ix_ix::ADD_IX_IX::new()),
                    0x2A => Box::new(ld::ld_ix_pnn::LD_IX_PNN::new(memory, pos)?),
                    0x2B => Box::new(math::dec::dec_ix::DEC_IX::new()),
                    0x34 => Box::new(math::inc::inc_pixd::INC_PIXD::new(memory, pos)?),
                    0x35 => Box::new(math::dec::dec_pixd::DEC_PIXD::new(memory, pos)?),
                    0x36 => Box::new(ld::ld_pixd_n::LD_PIXD_N::new(memory, pos)?),
                    0x39 => Box::new(math::add::add_ix_sp::ADD_IX_SP::new()),
                    0x46 => Box::new(ld::LD_B_PIXD::new(memory, pos)?),
                    0x4e => Box::new(ld::LD_C_PIXD::new(memory, pos)?),
                    0x56 => Box::new(ld::LD_D_PIXD::new(memory, pos)?),
                    0x5e => Box::new(ld::LD_E_PIXD::new(memory, pos)?),
                    0x66 => Box::new(ld::LD_H_PIXD::new(memory, pos)?),
                    0x6e => Box::new(ld::LD_L_PIXD::new(memory, pos)?),
                    0x70 => Box::new(ld::LD_PIXD_B::new(memory, pos)?),
                    0x71 => Box::new(ld::LD_PIXD_C::new(memory, pos)?),
                    0x72 => Box::new(ld::LD_PIXD_D::new(memory, pos)?),
                    0x73 => Box::new(ld::LD_PIXD_E::new(memory, pos)?),
                    0x74 => Box::new(ld::LD_PIXD_H::new(memory, pos)?),
                    0x75 => Box::new(ld::LD_PIXD_L::new(memory, pos)?),
                    0x77 => Box::new(ld::LD_PIXD_A::new(memory, pos)?),
                    0x7e => Box::new(ld::LD_A_PIXD::new(memory, pos)?),
                    0x86 => Box::new(math::add::add_a_pixd::ADD_A_PIXD::new(memory, pos)?),
                    0x8e => Box::new(math::adc::adc_a_pixd::ADC_A_PIXD::new(memory, pos)?),
                    0x96 => Box::new(math::sub::sub_pixd::SUB_PIXD::new(memory, pos)?),
                    0x9e => Box::new(math::sbc::sbc_a_pixd::SBC_A_PIXD::new(memory, pos)?),
                    0xA6 => Box::new(math::and::and_pixd::AND_PIXD::new(memory, pos)?),
                    0xAE => Box::new(math::xor::xor_pixd::XOR_PIXD::new(memory, pos)?),
                    0xB6 => Box::new(math::or::or_pixd::OR_PIXD::new(memory, pos)?),
                    0xBE => Box::new(math::cp::cp_pixd::CP_PIXD::new(memory, pos)?),
                    0xCB => {
                        let ins_byte3 = memory.read_8(pos.wrapping_add(3))?;
                        match ins_byte3 {
                            0x06 => Box::new(bit::rlc::rlc_pixd::RLC_PIXD::new(memory, pos)?),
                            0x0E => Box::new(bit::rrc::rrc_pixd::RRC_PIXD::new(memory, pos)?),
                            0x16 => Box::new(bit::rl::rl_pixd::RL_PIXD::new(memory, pos)?),
                            0x1E => Box::new(bit::rr::rr_pixd::RR_PIXD::new(memory, pos)?),
                            0x26 => Box::new(bit::sla::sla_pixd::SLA_PIXD::new(memory, pos)?),
                            0x2E => Box::new(bit::sra::sra_pixd::SRA_PIXD::new(memory, pos)?),
                            0x36 => Box::new(bit::sll::sll_pixd::SLL_PIXD::new(memory, pos)?),
                            0x3E => Box::new(bit::srl::srl_pixd::SRL_PIXD::new(memory, pos)?),
                            0x46 => Box::new(bit::bit::BIT_0_PIXD::new(memory, pos)?),
                            0x4E => Box::new(bit::bit::BIT_1_PIXD::new(memory, pos)?),
                            0x56 => Box::new(bit::bit::BIT_2_PIXD::new(memory, pos)?),
                            0x5E => Box::new(bit::bit::BIT_3_PIXD::new(memory, pos)?),
                            0x66 => Box::new(bit::bit::BIT_4_PIXD::new(memory, pos)?),
                            0x6E => Box::new(bit::bit::BIT_5_PIXD::new(memory, pos)?),
                            0x76 => Box::new(bit::bit::BIT_6_PIXD::new(memory, pos)?),
                            0x7E => Box::new(bit::bit::BIT_7_PIXD::new(memory, pos)?),
                            0x86 => Box::new(bit::res::RES_0_PIXD::new(memory, pos)?),
                            0x8E => Box::new(bit::res::RES_1_PIXD::new(memory, pos)?),
                            0x96 => Box::new(bit::res::RES_2_PIXD::new(memory, pos)?),
                            0x9E => Box::new(bit::res::RES_3_PIXD::new(memory, pos)?),
                            0xA6 => Box::new(bit::res::RES_4_PIXD::new(memory, pos)?),
                            0xAE => Box::new(bit::res::RES_5_PIXD::new(memory, pos)?),
                            0xB6 => Box::new(bit::res::RES_6_PIXD::new(memory, pos)?),
                            0xBE => Box::new(bit::res::RES_7_PIXD::new(memory, pos)?),
                            0xC6 => Box::new(bit::set::SET_0_PIXD::new(memory, pos)?),
                            0xCE => Box::new(bit::set::SET_1_PIXD::new(memory, pos)?),
                            0xD6 => Box::new(bit::set::SET_2_PIXD::new(memory, pos)?),
                            0xDE => Box::new(bit::set::SET_3_PIXD::new(memory, pos)?),
                            0xE6 => Box::new(bit::set::SET_4_PIXD::new(memory, pos)?),
                            0xEE => Box::new(bit::set::SET_5_PIXD::new(memory, pos)?),
                            0xF6 => Box::new(bit::set::SET_6_PIXD::new(memory, pos)?),
                            0xFE => Box::new(bit::set::SET_7_PIXD::new(memory, pos)?),
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid IX BIT instruction: 0x{:02x}",
                                    ins_byte3
                                )))
                            }
                        }
                    }
                    0xE1 => Box::new(stack::pop::pop_ix::POP_IX::new()),
                    0xE3 => Box::new(ex::ex_psp_ix::EX_PSP_IX::new()),
                    0xE5 => Box::new(stack::push::push_ix::PUSH_IX::new()),
                    0xE9 => Box::new(jump::jp::jp_pix::JP_PIX::new()),
                    0xF9 => Box::new(ld::ld_sp_ix::LD_SP_IX::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid IX instruction: 0x{:02x}",
                            ins_byte1
                        )))
                    }
                }
            }
            0xDE => Box::new(math::sbc::sbc_a_n::SBC_A_N::new(memory, pos)?),
            0xDF => Box::new(rst::RST_0x18::new()),
            0xE0 => Box::new(ret::ret_po::RET_PO::new()),
            0xE1 => Box::new(stack::pop::POP_HL::new()),
            0xE2 => Box::new(jump::jp::jp_po_nn::JP_PO_NN::new(memory, pos)?),
            0xE3 => Box::new(ex::ex_psp_hl::EX_PSP_HL::new()),
            0xE4 => Box::new(call::call_po_nn::CALL_PO_NN::new(memory, pos)?),
            0xE5 => Box::new(stack::push::PUSH_HL::new()),
            0xE6 => Box::new(math::and::and_n::AND_N::new(memory, pos)?),
            0xE7 => Box::new(rst::RST_0x20::new()),
            0xE8 => Box::new(ret::ret_pe::RET_PE::new()),
            0xE9 => Box::new(jump::jp::jp_phl::JP_PHL::new()),
            0xEA => Box::new(jump::jp::jp_pe_nn::JP_PE_NN::new(memory, pos)?),
            0xEB => Box::new(ex::ex_de_hl::EX_DE_HL::new()),
            0xEC => Box::new(call::call_pe_nn::CALL_PE_NN::new(memory, pos)?),
            0xED => {
                let ins_byte1 = memory.read_8(pos.wrapping_add(1))?;
                match ins_byte1 {
                    0x40 => Box::new(io::in_b_c::IN_B_C::new()),
                    0x41 => Box::new(io::out_c_b::OUT_C_B::new()),
                    0x42 => Box::new(math::sbc::sbc_hl_bc::SBC_HL_BC::new()),
                    0x43 => Box::new(ld::ld_pnn_bc_misc::LD_PNN_BC::new(memory, pos)?),
                    0x44 => Box::new(neg::NEG::new()),
                    0x45 => Box::new(retn::RETN::new()),
                    0x47 => Box::new(ld::ld_i_a::LD_I_A::new()),
                    0x48 => Box::new(io::in_c_c::IN_C_C::new()),
                    0x49 => Box::new(io::out_c_c::OUT_C_C::new()),
                    0x4B => Box::new(ld::LD_MISC_BC_PNN::new(memory, pos)?),
                    0x50 => Box::new(io::in_d_c::IN_D_C::new()),
                    0x51 => Box::new(io::out_c_d::OUT_C_D::new()),
                    0x52 => Box::new(math::sbc::sbc_hl_de::SBC_HL_DE::new()),
                    0x53 => Box::new(ld::ld_pnn_de_misc::LD_PNN_DE::new(memory, pos)?),
                    0x57 => Box::new(ld::ld_a_i::LD_A_I::new()),
                    0x58 => Box::new(io::in_e_c::IN_E_C::new()),
                    0x59 => Box::new(io::out_c_e::OUT_C_E::new()),
                    0x5B => Box::new(ld::LD_MISC_DE_PNN::new(memory, pos)?),
                    0x60 => Box::new(io::in_h_c::IN_H_C::new()),
                    0x61 => Box::new(io::out_c_h::OUT_C_H::new()),
                    0x62 => Box::new(math::sbc::sbc_hl_hl::SBC_HL_HL::new()),
                    // 0x63 => Box::new(ld::ld_pnn_hl_misc::LD_PNN_HL::new(memory, pos)?),
                    0x68 => Box::new(io::in_l_c::IN_L_C::new()),
                    0x69 => Box::new(io::out_c_l::OUT_C_L::new()),
                    0x6B => Box::new(ld::LD_MISC_HL_PNN::new(memory, pos)?),
                    0x72 => Box::new(math::sbc::sbc_hl_sp::SBC_HL_SP::new()),
                    0x73 => Box::new(ld::ld_pnn_sp::LD_PNN_SP::new(memory, pos)?),
                    0x78 => Box::new(io::in_a_c::IN_A_C::new()),
                    0x79 => Box::new(io::out_c_a::OUT_C_A::new()),
                    0x7B => Box::new(ld::ld_sp_pnn::LD_MISC_SP_PNN::new(memory, pos)?),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid MISC instruction: 0x{:02x}",
                            ins_byte1
                        )))
                    }
                }
            }
            0xEE => Box::new(math::xor::xor_n::XOR_N::new(memory, pos)?),
            0xEF => Box::new(rst::RST_0x28::new()),
            0xF0 => Box::new(ret::ret_p::RET_P::new()),
            0xF1 => Box::new(stack::pop::POP_AF::new()),
            0xF2 => Box::new(jump::jp::jp_p_nn::JP_P_NN::new(memory, pos)?),
            0xF3 => Box::new(di::DI::new()),
            0xF4 => Box::new(call::call_p_nn::CALL_P_NN::new(memory, pos)?),
            0xF5 => Box::new(stack::push::PUSH_AF::new()),
            0xF6 => Box::new(math::or::or_n::OR_N::new(memory, pos)?),
            0xF7 => Box::new(rst::RST_0x30::new()),
            0xF8 => Box::new(ret::ret_m::RET_M::new()),
            0xF9 => Box::new(ld::ld_sp_hl::LD_SP_HL::new()),
            0xFA => Box::new(jump::jp::jp_m_nn::JP_M_NN::new(memory, pos)?),
            0xFB => Box::new(ei::EI::new()),
            0xFC => Box::new(call::call_m_nn::CALL_M_NN::new(memory, pos)?),
            0xFD => {
                let ins_byte1 = memory.read_8(pos.wrapping_add(1))?;
                match ins_byte1 {
                    0x09 => Box::new(math::add::add_iy_bc::ADD_IY_BC::new()),
                    0x19 => Box::new(math::add::add_iy_de::ADD_IY_DE::new()),
                    0x21 => Box::new(ld::ld_iy_nn::LD_IY_NN::new(memory, pos)?),
                    0x22 => Box::new(ld::ld_pnn_iy::LD_PNN_IY::new(memory, pos)?),
                    0x23 => Box::new(math::inc::inc_iy::INC_IY::new()),
                    0x2A => Box::new(ld::ld_iy_pnn::LD_IY_PNN::new(memory, pos)?),
                    0x2B => Box::new(math::dec::dec_iy::DEC_IY::new()),
                    0x29 => Box::new(math::add::add_iy_iy::ADD_IY_IY::new()),
                    0x39 => Box::new(math::add::add_iy_sp::ADD_IY_SP::new()),
                    0x46 => Box::new(ld::LD_B_PIYD::new(memory, pos)?),
                    0x4e => Box::new(ld::LD_C_PIYD::new(memory, pos)?),
                    0x56 => Box::new(ld::LD_D_PIYD::new(memory, pos)?),
                    0x5e => Box::new(ld::LD_E_PIYD::new(memory, pos)?),
                    0x66 => Box::new(ld::LD_H_PIYD::new(memory, pos)?),
                    0x6e => Box::new(ld::LD_L_PIYD::new(memory, pos)?),
                    0x70 => Box::new(ld::LD_PIYD_B::new(memory, pos)?),
                    0x71 => Box::new(ld::LD_PIYD_C::new(memory, pos)?),
                    0x72 => Box::new(ld::LD_PIYD_D::new(memory, pos)?),
                    0x73 => Box::new(ld::LD_PIYD_E::new(memory, pos)?),
                    0x74 => Box::new(ld::LD_PIYD_H::new(memory, pos)?),
                    0x75 => Box::new(ld::LD_PIYD_L::new(memory, pos)?),
                    0x77 => Box::new(ld::LD_PIYD_A::new(memory, pos)?),
                    0x7e => Box::new(ld::LD_A_PIYD::new(memory, pos)?),
                    0x86 => Box::new(math::add::add_a_piyd::ADD_A_PIYD::new(memory, pos)?),
                    0x8E => Box::new(math::adc::adc_a_piyd::ADC_A_PIYD::new(memory, pos)?),
                    0x96 => Box::new(math::sub::sub_piyd::SUB_PIYD::new(memory, pos)?),
                    0x9E => Box::new(math::sbc::sbc_a_piyd::SBC_A_PIYD::new(memory, pos)?),
                    0xA6 => Box::new(math::and::and_piyd::AND_PIYD::new(memory, pos)?),
                    0xAE => Box::new(math::xor::xor_piyd::XOR_PIYD::new(memory, pos)?),
                    0xB6 => Box::new(math::or::or_piyd::OR_PIYD::new(memory, pos)?),
                    0xBE => Box::new(math::cp::cp_piyd::CP_PIYD::new(memory, pos)?),
                    0xCB => {
                        let ins_byte3 = memory.read_8(pos.wrapping_add(3))?;
                        match ins_byte3 {
                            0x06 => Box::new(bit::rlc::rlc_piyd::RLC_PIYD::new(memory, pos)?),
                            0x0E => Box::new(bit::rrc::rrc_piyd::RRC_PIYD::new(memory, pos)?),
                            0x16 => Box::new(bit::rl::rl_piyd::RL_PIYD::new(memory, pos)?),
                            0x1E => Box::new(bit::rr::rr_piyd::RR_PIYD::new(memory, pos)?),
                            0x26 => Box::new(bit::sla::sla_piyd::SLA_PIYD::new(memory, pos)?),
                            0x2E => Box::new(bit::sra::sra_piyd::SRA_PIYD::new(memory, pos)?),
                            0x36 => Box::new(bit::sll::sll_piyd::SLL_PIYD::new(memory, pos)?),
                            0x3E => Box::new(bit::srl::srl_piyd::SRL_PIYD::new(memory, pos)?),
                            0x46 => Box::new(bit::bit::BIT_0_PIYD::new(memory, pos)?),
                            0x4E => Box::new(bit::bit::BIT_1_PIYD::new(memory, pos)?),
                            0x56 => Box::new(bit::bit::BIT_2_PIYD::new(memory, pos)?),
                            0x5E => Box::new(bit::bit::BIT_3_PIYD::new(memory, pos)?),
                            0x66 => Box::new(bit::bit::BIT_4_PIYD::new(memory, pos)?),
                            0x6E => Box::new(bit::bit::BIT_5_PIYD::new(memory, pos)?),
                            0x76 => Box::new(bit::bit::BIT_6_PIYD::new(memory, pos)?),
                            0x7E => Box::new(bit::bit::BIT_7_PIYD::new(memory, pos)?),
                            0x86 => Box::new(bit::res::RES_0_PIYD::new(memory, pos)?),
                            0x8E => Box::new(bit::res::RES_1_PIYD::new(memory, pos)?),
                            0x96 => Box::new(bit::res::RES_2_PIYD::new(memory, pos)?),
                            0x9E => Box::new(bit::res::RES_3_PIYD::new(memory, pos)?),
                            0xA6 => Box::new(bit::res::RES_4_PIYD::new(memory, pos)?),
                            0xAE => Box::new(bit::res::RES_5_PIYD::new(memory, pos)?),
                            0xB6 => Box::new(bit::res::RES_6_PIYD::new(memory, pos)?),
                            0xBE => Box::new(bit::res::RES_7_PIYD::new(memory, pos)?),
                            0xC6 => Box::new(bit::set::SET_0_PIYD::new(memory, pos)?),
                            0xCE => Box::new(bit::set::SET_1_PIYD::new(memory, pos)?),
                            0xD6 => Box::new(bit::set::SET_2_PIYD::new(memory, pos)?),
                            0xDE => Box::new(bit::set::SET_3_PIYD::new(memory, pos)?),
                            0xE6 => Box::new(bit::set::SET_4_PIYD::new(memory, pos)?),
                            0xEE => Box::new(bit::set::SET_5_PIYD::new(memory, pos)?),
                            0xF6 => Box::new(bit::set::SET_6_PIYD::new(memory, pos)?),
                            0xFE => Box::new(bit::set::SET_7_PIYD::new(memory, pos)?),
                            _ => {
                                return Err(ParseError::InvalidInstruction(format!(
                                    "Invalid IY bit instruction: 0x{:02x}",
                                    ins_byte3
                                )))
                            }
                        }
                    }
                    0xE1 => Box::new(stack::pop::pop_iy::POP_IY::new()),
                    0xE3 => Box::new(ex::ex_psp_iy::EX_PSP_IY::new()),
                    0xE5 => Box::new(stack::push::push_iy::PUSH_IY::new()),
                    0xE9 => Box::new(jump::jp::jp_piy::JP_PIY::new()),
                    0xF9 => Box::new(ld::ld_sp_iy::LD_SP_IY::new()),
                    _ => {
                        return Err(ParseError::InvalidInstruction(format!(
                            "Invalid IY instruction 0x{:02x}",
                            ins_byte1
                        )))
                    }
                }
            }
            0xFE => Box::new(math::cp::cp_n::CP_N::new(memory, pos)?),
            0xFF => Box::new(rst::RST_0x38::new()),
        };
        Ok(instruction)
    }
}

pub const Z80_PARSER: Z80Parser = Z80Parser {};
