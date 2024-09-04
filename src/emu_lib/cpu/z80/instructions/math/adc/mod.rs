mod generics;

// macro_rules! adc_rr_rr {
//     ($reg1:expr, $reg2:expr, $flags:expr) => {
//         let result = $reg1
//             .wrapping_add($reg2)
//             .wrapping_add($flags.carry() as u16);
//         let carry = result < *$reg1;
//         // check for carry between bits 11 and 12
//         let half_carry = ((*$reg1 & 0x0fff) + ($reg2 & 0x0fff)) > 0x0fff;
//         *$reg1 = result;
//         $flags.set_carry(carry);
//         $flags.set_half_carry(half_carry);
//         $flags.set_add_sub(false);
//         //set undocumented flags
//         $flags.set_bit3((result >> 11) & 1 == 1);
//         $flags.set_bit5((result >> 13) & 1 == 1);
//     };
// }
//
// pub(crate) use adc_rr_rr;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::adc::generics::adc_r_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;
use std::fmt::Display;
pub mod adc_a_n;

generics::adc_r_r::adc_r_r!(a, b, "88", "A", "B");
generics::adc_r_r::adc_r_r!(a, c, "89", "A", "C");
generics::adc_r_r::adc_r_r!(a, d, "8a", "A", "D");
generics::adc_r_r::adc_r_r!(a, e, "8b", "A", "E");
generics::adc_r_r::adc_r_r!(a, h, "8c", "A", "H");
generics::adc_r_r::adc_r_r!(a, l, "8d", "A", "L");
