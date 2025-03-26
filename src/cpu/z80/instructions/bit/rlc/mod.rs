use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::rlc::generics::rlc_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod rlc_phl;
pub mod rlc_pixd;
pub mod rlc_piyd;

generics::rlc_r::rlc_r!(b, "00", "B");
generics::rlc_r::rlc_r!(c, "01", "C");
generics::rlc_r::rlc_r!(d, "02", "D");
generics::rlc_r::rlc_r!(e, "03", "E");
generics::rlc_r::rlc_r!(h, "04", "H");
generics::rlc_r::rlc_r!(l, "05", "L");
generics::rlc_r::rlc_r!(a, "07", "A");
