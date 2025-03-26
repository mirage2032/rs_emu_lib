use std::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::sll::generics::sll_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;

mod generics;
pub mod sll_phl;
pub mod sll_pixd;
pub mod sll_piyd;

generics::sll_r::sll_r!(b, "30", "B");
generics::sll_r::sll_r!(c, "31", "C");
generics::sll_r::sll_r!(d, "32", "D");
generics::sll_r::sll_r!(e, "33", "E");
generics::sll_r::sll_r!(h, "34", "H");
generics::sll_r::sll_r!(l, "35", "L");
generics::sll_r::sll_r!(a, "37", "A");
