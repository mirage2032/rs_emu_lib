use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::add::generics::{add_r_r_setf, add_rr_rr_setf};
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

pub mod add_a_phl;
pub mod add_a_pixd;
pub mod add_hl_sp;
pub mod add_ix_sp;
pub mod add_a_n;
mod generics;
pub mod add_ix_bc;
pub mod add_ix_de;
pub mod add_ix_ix;
pub mod add_iy_bc;
pub mod add_iy_de;
pub mod add_iy_iy;
pub mod add_iy_sp;

generics::add_r_r::add_r_r!(a, b, "80", "A", "B");
generics::add_r_r::add_r_r!(a, c, "81", "A", "C");
generics::add_r_r::add_r_r!(a, d, "82", "A", "D");
generics::add_r_r::add_r_r!(a, e, "83", "A", "E");
generics::add_r_r::add_r_r!(a, h, "84", "A", "H");
generics::add_r_r::add_r_r!(a, l, "85", "A", "L");
generics::add_r_r::add_r_r!(a, a, "87", "A", "A");

generics::add_rr_rr::add_rr_rr!(hl, bc, "09", "HL", "BC");
generics::add_rr_rr::add_rr_rr!(hl, de, "19", "HL", "DE");
generics::add_rr_rr::add_rr_rr!(hl, hl, "29", "HL", "HL");
