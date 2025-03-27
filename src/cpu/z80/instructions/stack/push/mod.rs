use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::push_16;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

mod generics;
pub mod push_ix;
pub mod push_iy;

generics::push_rr::push_rr!(bc, "c5", "BC");
generics::push_rr::push_rr!(de, "d5", "DE");
generics::push_rr::push_rr!(hl, "e5", "HL");
generics::push_rr::push_rr!(af, "f5", "AF");
