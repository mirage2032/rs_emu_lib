use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::xor::generics::xor_r_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod xor_n;

generics::xor_r::xor_r!(b, "a8", "B");
generics::xor_r::xor_r!(c, "a9", "C");
generics::xor_r::xor_r!(d, "aa", "D");
generics::xor_r::xor_r!(e, "ab", "E");
generics::xor_r::xor_r!(h, "ac", "H");
generics::xor_r::xor_r!(l, "ad", "L");
generics::xor_r::xor_r!(a, "af", "A");
