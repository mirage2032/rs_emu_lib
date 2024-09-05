mod generics;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::BaseRegister;
use crate::cpu::z80::instructions::bit::sla::generics::sla_r_setf;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;
use std::fmt::Display;

pub mod sla_phl;
pub mod sla_pixd;

generics::sla_r::sla_r!(b, "20", "B");
generics::sla_r::sla_r!(c, "21", "C");
generics::sla_r::sla_r!(d, "22", "D");
generics::sla_r::sla_r!(e, "23", "E");
generics::sla_r::sla_r!(h, "24", "H");
generics::sla_r::sla_r!(l, "25", "L");
generics::sla_r::sla_r!(a, "27", "A");