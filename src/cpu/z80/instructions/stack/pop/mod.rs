use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::BaseInstruction;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::instruction::pop_16;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

pub mod pop_ix;

mod generics;
generics::pop_rr::pop_rr!(bc, "c1", "BC");
generics::pop_rr::pop_rr!(de, "d1", "DE");
generics::pop_rr::pop_rr!(hl, "e1", "HL");
generics::pop_rr::pop_rr!(af, "f1", "AF");
