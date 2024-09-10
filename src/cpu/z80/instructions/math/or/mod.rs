use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::or::generics::or_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod or_ixd;
pub mod or_n;
pub mod or_phl;

generics::or_r::or_r!(b, "b0", "B");
generics::or_r::or_r!(c, "b1", "C");
generics::or_r::or_r!(d, "b2", "D");
generics::or_r::or_r!(e, "b3", "E");
generics::or_r::or_r!(h, "b4", "H");
generics::or_r::or_r!(l, "b5", "L");
generics::or_r::or_r!(a, "b7", "A");
