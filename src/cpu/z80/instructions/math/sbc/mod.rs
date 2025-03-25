use crate::cpu::ExecutableInstruction;
use crate::cpu::BaseInstruction;
use crate::cpu::z80::Z80;
use crate::memory::Memory;
use crate::cpu::instruction::InstructionCommon;
use crate::io::IO;
use crate::cpu::z80::instructions::math::sbc::generics::sbc_r_r;
use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::z80::instructions::math::sbc::generics::sbc_a_r::sbc_a_r;

pub mod generics;
pub mod sbc_a_n;
pub mod sbc_a_phl;

sbc_a_r!(b,"98", "B");
sbc_a_r!(c,"99", "C");
sbc_a_r!(d,"9a", "D");
sbc_a_r!(e,"9b", "E");
sbc_a_r!(h,"9c", "H");
sbc_a_r!(l,"9d", "L");
sbc_a_r!(a,"9f", "A");