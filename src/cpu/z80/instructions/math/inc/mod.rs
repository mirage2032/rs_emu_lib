use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::inc::generics::inc_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod inc_phl;
pub mod inc_sp;
pub mod inc_ix;
pub mod inc_pixd;
pub mod inc_iy;

generics::inc_r::inc_r!(b, "04", "B");
generics::inc_r::inc_r!(c, "0c", "C");
generics::inc_r::inc_r!(d, "14", "D");
generics::inc_r::inc_r!(e, "1c", "E");
generics::inc_r::inc_r!(h, "24", "H");
generics::inc_r::inc_r!(l, "2c", "L");
generics::inc_r::inc_r!(a, "3c", "A");

generics::inc_rr::inc_rr!(bc, "03", "BC");
generics::inc_rr::inc_rr!(de, "13", "DE");
generics::inc_rr::inc_rr!(hl, "23", "HL");
