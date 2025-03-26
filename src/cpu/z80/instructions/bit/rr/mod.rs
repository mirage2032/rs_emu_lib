use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::rr::generics::rr_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod rr_phl;
pub mod rr_pixd;
pub mod rr_piyd;

generics::rr_r::rr_r!(b, "18", "B");
generics::rr_r::rr_r!(c, "19", "C");
generics::rr_r::rr_r!(d, "1a", "D");
generics::rr_r::rr_r!(e, "1b", "E");
generics::rr_r::rr_r!(h, "1c", "H");
generics::rr_r::rr_r!(l, "1d", "L");
generics::rr_r::rr_r!(a, "1f", "A");
