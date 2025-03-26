use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::cp::generics::cp_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

pub mod cp_n;
pub mod cp_phl;
mod generics;
pub mod cp_pixd;

generics::cp_r::cp_r!(b, "b8", "B");
generics::cp_r::cp_r!(c, "b9", "C");
generics::cp_r::cp_r!(d, "ba", "D");
generics::cp_r::cp_r!(e, "bb", "E");
generics::cp_r::cp_r!(h, "bc", "H");
generics::cp_r::cp_r!(l, "bd", "L");
generics::cp_r::cp_r!(a, "bf", "A");
