use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::adc::generics::adc_r_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;

pub mod adc_a_n;
pub mod adc_a_phl;
pub mod adc_a_pixd;

generics::adc_r_r::adc_r_r!(a, b, "88", "A", "B");
generics::adc_r_r::adc_r_r!(a, c, "89", "A", "C");
generics::adc_r_r::adc_r_r!(a, d, "8a", "A", "D");
generics::adc_r_r::adc_r_r!(a, e, "8b", "A", "E");
generics::adc_r_r::adc_r_r!(a, h, "8c", "A", "H");
generics::adc_r_r::adc_r_r!(a, l, "8d", "A", "L");
