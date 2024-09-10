use core::fmt;
use std::fmt::Display;

use hex_literal::hex;

use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

pub mod generics;
pub mod ld_ix_nn;
pub mod ld_misc_sp_pnn;
pub mod ld_pixd_n;
pub mod ld_sp_hl;
pub mod ld_sp_ix;
pub mod ld_sp_nn;

generics::ld_r_r::ld_r_r!(a, b, "78", "A", "B");
generics::ld_r_r::ld_r_r!(a, c, "79", "A", "C");
generics::ld_r_r::ld_r_r!(a, d, "7a", "A", "D");
generics::ld_r_r::ld_r_r!(a, e, "7b", "A", "E");
generics::ld_r_r::ld_r_r!(a, h, "7c", "A", "H");
generics::ld_r_r::ld_r_r!(a, l, "7d", "A", "L");

// generics::ld_r_r::ld_r_r!(b, b, "40", "B", "B");
generics::ld_r_r::ld_r_r!(b, c, "41", "B", "C");
generics::ld_r_r::ld_r_r!(b, d, "42", "B", "D");
generics::ld_r_r::ld_r_r!(b, e, "43", "B", "E");
generics::ld_r_r::ld_r_r!(b, h, "44", "B", "H");
generics::ld_r_r::ld_r_r!(b, l, "45", "B", "L");
generics::ld_r_r::ld_r_r!(b, a, "47", "B", "A");

generics::ld_r_r::ld_r_r!(c, b, "48", "C", "B");
// generics::ld_r_r::ld_r_r!(c, c "49", "C", "C");
generics::ld_r_r::ld_r_r!(c, d, "4a", "C", "D");
generics::ld_r_r::ld_r_r!(c, e, "4b", "C", "E");
generics::ld_r_r::ld_r_r!(c, h, "4c", "C", "H");
generics::ld_r_r::ld_r_r!(c, l, "4d", "C", "L");
generics::ld_r_r::ld_r_r!(c, a, "4f", "C", "A");

generics::ld_r_r::ld_r_r!(d, b, "50", "D", "B");
generics::ld_r_r::ld_r_r!(d, c, "51", "D", "C");
// generics::ld_r_r::ld_r_r!(d, d, "52", "D", "D");
generics::ld_r_r::ld_r_r!(d, e, "53", "D", "E");
generics::ld_r_r::ld_r_r!(d, h, "54", "D", "H");
generics::ld_r_r::ld_r_r!(d, l, "55", "D", "L");
generics::ld_r_r::ld_r_r!(d, a, "57", "D", "A");

generics::ld_r_r::ld_r_r!(e, b, "58", "E", "B");
generics::ld_r_r::ld_r_r!(e, c, "59", "E", "C");
generics::ld_r_r::ld_r_r!(e, d, "5a", "E", "D");
// generics::ld_r_r::ld_r_r!(e, e, "5b", "E", "E");
generics::ld_r_r::ld_r_r!(e, h, "5c", "E", "H");
generics::ld_r_r::ld_r_r!(e, l, "5d", "E", "L");
generics::ld_r_r::ld_r_r!(e, a, "5f", "E", "A");

generics::ld_r_r::ld_r_r!(h, b, "60", "H", "B");
generics::ld_r_r::ld_r_r!(h, c, "61", "H", "C");
generics::ld_r_r::ld_r_r!(h, d, "62", "H", "D");
generics::ld_r_r::ld_r_r!(h, e, "63", "H", "E");
// generics::ld_r_r::ld_r_r!(h, h, "64", "H", "H");
generics::ld_r_r::ld_r_r!(h, l, "65", "H", "L");
generics::ld_r_r::ld_r_r!(h, a, "67", "H", "A");

generics::ld_r_r::ld_r_r!(l, b, "68", "L", "B");
generics::ld_r_r::ld_r_r!(l, c, "69", "L", "C");
generics::ld_r_r::ld_r_r!(l, d, "6a", "L", "D");
generics::ld_r_r::ld_r_r!(l, e, "6b", "L", "E");
generics::ld_r_r::ld_r_r!(l, h, "6c", "L", "H");
// generics::ld_r_r::ld_r_r!(l, l, "6d", "L", "L");
generics::ld_r_r::ld_r_r!(l, a, "6f", "L", "A");

generics::ld_r_n::ld_r_n!(b, "06", "B");
generics::ld_r_n::ld_r_n!(c, "0e", "C");
generics::ld_r_n::ld_r_n!(d, "16", "D");
generics::ld_r_n::ld_r_n!(e, "1e", "E");
generics::ld_r_n::ld_r_n!(h, "26", "H");
generics::ld_r_n::ld_r_n!(l, "2e", "L");
generics::ld_r_n::ld_r_n!(a, "3e", "A");

generics::ld_rr_pnn::ld_rr_pnn!(hl, "2a", "HL");

generics::ld_misc_rr_pnn::ld_misc_rr_pnn!(bc, "4b", "BC");
generics::ld_misc_rr_pnn::ld_misc_rr_pnn!(de, "5b", "DE");
generics::ld_misc_rr_pnn::ld_misc_rr_pnn!(hl, "6b", "HL"); //undoc

generics::ld_r_pnn::ld_r_pnn!(a, "3a", "A");

generics::ld_r_prr::ld_r_prr!(a, bc, "0a", "A", "BC");
generics::ld_r_prr::ld_r_prr!(a, de, "1a", "A", "DE");

generics::ld_r_prr::ld_r_prr!(a, hl, "7e", "A", "HL");
generics::ld_r_prr::ld_r_prr!(b, hl, "46", "B", "HL");
generics::ld_r_prr::ld_r_prr!(c, hl, "4e", "C", "HL");
generics::ld_r_prr::ld_r_prr!(d, hl, "56", "D", "HL");
generics::ld_r_prr::ld_r_prr!(e, hl, "5e", "E", "HL");
generics::ld_r_prr::ld_r_prr!(h, hl, "66", "H", "HL");
generics::ld_r_prr::ld_r_prr!(l, hl, "6e", "L", "HL");

generics::ld_rr_nn::ld_rr_nn!(bc, "01", "BC");
generics::ld_rr_nn::ld_rr_nn!(de, "11", "DE");
generics::ld_rr_nn::ld_rr_nn!(hl, "21", "HL");

generics::ld_prr_r::ld_prr_r!(bc, a, "02", "BC", "A");
generics::ld_prr_r::ld_prr_r!(de, a, "12", "DE", "A");
generics::ld_prr_r::ld_prr_r!(hl, a, "71", "HL", "A");
generics::ld_prr_r::ld_prr_r!(hl, b, "70", "HL", "B");
generics::ld_prr_r::ld_prr_r!(hl, c, "71", "HL", "C");
generics::ld_prr_r::ld_prr_r!(hl, d, "72", "HL", "D");
generics::ld_prr_r::ld_prr_r!(hl, e, "73", "HL", "E");
generics::ld_prr_r::ld_prr_r!(hl, h, "74", "HL", "H");
generics::ld_prr_r::ld_prr_r!(hl, l, "75", "HL", "L");

generics::ld_pnn_r::ld_pnn_r!(a, "32", "A");
generics::ld_pnn_rr::ld_pnn_rr!(hl, "22", "HL");

generics::ld_prr_n::ld_prr_n!(hl, "36", "HL");

generics::ld_pixd_r::ld_pixd_r!(b, "70", "B");
generics::ld_pixd_r::ld_pixd_r!(c, "71", "C");
generics::ld_pixd_r::ld_pixd_r!(d, "72", "D");
generics::ld_pixd_r::ld_pixd_r!(e, "73", "E");
generics::ld_pixd_r::ld_pixd_r!(h, "74", "H");
generics::ld_pixd_r::ld_pixd_r!(l, "75", "L");
generics::ld_pixd_r::ld_pixd_r!(a, "77", "A");

generics::ld_r_pixd::ld_r_pixd!(b, "46", "B");
generics::ld_r_pixd::ld_r_pixd!(c, "4e", "C");
generics::ld_r_pixd::ld_r_pixd!(d, "56", "D");
generics::ld_r_pixd::ld_r_pixd!(e, "5e", "E");
generics::ld_r_pixd::ld_r_pixd!(h, "66", "H");
generics::ld_r_pixd::ld_r_pixd!(l, "6e", "L");
generics::ld_r_pixd::ld_r_pixd!(a, "7e", "A");
