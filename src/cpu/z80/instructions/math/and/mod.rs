use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::and::generics::and_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

pub mod and_pixd;
pub mod and_n;
pub mod and_phl;
mod generics;

generics::and_r::and_r!(b, "a0", "B");
generics::and_r::and_r!(c, "a1", "C");
generics::and_r::and_r!(d, "a2", "D");
generics::and_r::and_r!(e, "a3", "E");
generics::and_r::and_r!(h, "a4", "H");
generics::and_r::and_r!(l, "a5", "L");
generics::and_r::and_r!(a, "a7", "A");
