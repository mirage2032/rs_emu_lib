mod generics;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::BaseRegister;
use crate::cpu::z80::instructions::bit::rl::generics::rl_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;
use std::fmt::Display;

pub mod rl_phl;
pub mod rl_pixd;

generics::rl_r::rl_r!(b, "10", "B");
generics::rl_r::rl_r!(c, "11", "C");
generics::rl_r::rl_r!(d, "12", "D");
generics::rl_r::rl_r!(e, "13", "E");
generics::rl_r::rl_r!(h, "14", "H");
generics::rl_r::rl_r!(l, "15", "L");
generics::rl_r::rl_r!(a, "17", "A");