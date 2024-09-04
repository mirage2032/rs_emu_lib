mod generics;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::math::dec::generics::dec_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;
use std::fmt::Display;

pub mod dec_sp;

generics::dec_r::dec_r!(b, "05", "B");
generics::dec_r::dec_r!(c, "0d", "C");
generics::dec_r::dec_r!(d, "15", "D");
generics::dec_r::dec_r!(e, "1d", "E");
generics::dec_r::dec_r!(h, "25", "H");
generics::dec_r::dec_r!(l, "2d", "L");

generics::dec_rr::dec_rr!(bc, "0b", "BC");
generics::dec_rr::dec_rr!(de, "1b", "DE");
generics::dec_rr::dec_rr!(hl, "2b", "HL");
