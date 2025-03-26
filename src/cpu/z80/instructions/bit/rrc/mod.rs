use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::rrc::generics::rrc_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod rrc_phl;
pub mod rrc_pixd;
pub mod rrc_piyd;

generics::rrc_r::rrc_r!(b, "08", "B");
generics::rrc_r::rrc_r!(c, "09", "C");
generics::rrc_r::rrc_r!(d, "0a", "D");
generics::rrc_r::rrc_r!(e, "0b", "E");
generics::rrc_r::rrc_r!(h, "0c", "H");
generics::rrc_r::rrc_r!(l, "0d", "L");
generics::rrc_r::rrc_r!(a, "0f", "A");
