use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::sra::generics::sra_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod sra_phl;
pub mod sra_pixd;

generics::sra_r::sra_r!(b, "28", "B");
generics::sra_r::sra_r!(c, "29", "C");
generics::sra_r::sra_r!(d, "2a", "D");
generics::sra_r::sra_r!(e, "2b", "E");
generics::sra_r::sra_r!(h, "2c", "H");
generics::sra_r::sra_r!(l, "2d", "L");
generics::sra_r::sra_r!(a, "2f", "A");
