use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::sub::generics::sub_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod sub_ixd;
pub mod sub_n;
pub mod sub_phl;

generics::sub_r::sub_r!(b, "90", "B");
generics::sub_r::sub_r!(c, "91", "C");
generics::sub_r::sub_r!(d, "92", "D");
generics::sub_r::sub_r!(e, "93", "E");
generics::sub_r::sub_r!(h, "94", "H");
generics::sub_r::sub_r!(l, "95", "L");
generics::sub_r::sub_r!(a, "97", "A");
