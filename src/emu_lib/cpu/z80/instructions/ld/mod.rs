use core::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::emu_lib::cpu::instruction::InstructionCommon;
use crate::emu_lib::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

pub mod generics;
pub mod ld_ix_nn;
pub mod ld_ixpd_a;
pub mod ld_sp_hl;
pub mod ld_sp_ix;
pub mod ld_sp_nn;

generics::ld_r_r::ld_r_r!(a, c, "79", "A", "C");
generics::ld_r_r::ld_r_r!(b, a, "47", "B", "A");
generics::ld_r_r::ld_r_r!(b, d, "42", "B", "D");
generics::ld_r_r::ld_r_r!(c, a, "4f", "C", "A");
generics::ld_r_r::ld_r_r!(c, e, "4b", "C", "E");
generics::ld_r_r::ld_r_r!(d, a, "57", "D", "A");
generics::ld_r_r::ld_r_r!(e, a, "5f", "E", "A");

generics::ld_r_n::ld_r_n!(l, "2e", "L");
generics::ld_r_n::ld_r_n!(b, "06", "B");
generics::ld_r_n::ld_r_n!(c, "0e", "C");
generics::ld_r_n::ld_r_n!(d, "16", "D");
generics::ld_r_n::ld_r_n!(e, "1e", "E");
generics::ld_r_n::ld_r_n!(h, "26", "H");

generics::ld_rr_pnn::ld_rr_pnn!(hl, "2a", "HL");

generics::ld_r_prr::ld_r_prr!(a, bc, "0a", "A", "BC");
generics::ld_r_prr::ld_r_prr!(a, de, "1a", "A", "DE");
generics::ld_r_prr::ld_r_prr!(a, hl, "7e", "A", "HL");
generics::ld_r_prr::ld_r_prr!(e, hl, "5e", "E", "HL");

generics::ld_rr_nn::ld_rr_nn!(bc, "01", "BC");
generics::ld_rr_nn::ld_rr_nn!(de, "11", "DE");
generics::ld_rr_nn::ld_rr_nn!(hl, "21", "HL");

generics::ld_prr_r::ld_prr_r!(bc, a, "02", "BC", "A");
generics::ld_prr_r::ld_prr_r!(de, a, "12", "DE", "A");
generics::ld_prr_r::ld_prr_r!(hl, a, "77", "HL", "A");

generics::ld_pnn_r::ld_pnn_r!(a, "32", "A");
generics::ld_pnn_rr::ld_pnn_rr!(hl, "22", "HL");

generics::ld_prr_n::ld_prr_n!(hl, "36", "HL");
