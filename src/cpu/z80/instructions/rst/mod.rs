pub mod generic;

use hex_literal::hex;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{push_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::rst::generic::rst_n;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

rst_n!(0x00,"c7");
rst_n!(0x08,"cf");
rst_n!(0x10,"d7");
rst_n!(0x18,"df");
rst_n!(0x20,"e7");
rst_n!(0x28,"ef");
rst_n!(0x30,"f7");
rst_n!(0x38,"ff");