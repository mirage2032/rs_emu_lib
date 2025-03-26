use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::srl::generics::srl_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod srl_phl;
pub mod srl_pixd;
pub mod srl_piyd;
generics::srl_r::srl_r!(b, "38", "B");
generics::srl_r::srl_r!(c, "39", "C");
generics::srl_r::srl_r!(d, "3a", "D");
generics::srl_r::srl_r!(e, "3b", "E");
generics::srl_r::srl_r!(h, "3c", "H");
generics::srl_r::srl_r!(l, "3d", "L");
generics::srl_r::srl_r!(a, "3f", "A");
