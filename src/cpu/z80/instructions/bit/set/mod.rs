use std::fmt::Display;
use crate::memory::MemoryDevice;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::set::generics::set_b_r::set_b_r;
use crate::cpu::z80::instructions::bit::set::generics::set_b_phl::set_b_phl;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::cpu::ExecutableInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;

mod generics;

set_b_r!(0, b, "B", "c0");
set_b_r!(0, c, "C", "c1");
set_b_r!(0, d, "D", "c2");
set_b_r!(0, e, "E", "c3");
set_b_r!(0, h, "H", "c4");
set_b_r!(0, l, "L", "c5");
set_b_phl!(0, "c6");
set_b_r!(0, a, "A", "c7");

set_b_r!(1, b, "B", "c8");
set_b_r!(1, c, "C", "c9");
set_b_r!(1, d, "D", "ca");
set_b_r!(1, e, "E", "cb");
set_b_r!(1, h, "H", "cc");
set_b_r!(1, l, "L", "cd");
set_b_phl!(1, "ce");
set_b_r!(1, a, "A", "cf");

set_b_r!(2, b, "B", "d0");
set_b_r!(2, c, "C", "d1");
set_b_r!(2, d, "D", "d2");
set_b_r!(2, e, "E", "d3");
set_b_r!(2, h, "H", "d4");
set_b_r!(2, l, "L", "d5");
set_b_phl!(2, "d6");
set_b_r!(2, a, "A", "d7");

set_b_r!(3, b, "B", "d8");
set_b_r!(3, c, "C", "d9");
set_b_r!(3, d, "D", "da");
set_b_r!(3, e, "E", "db");
set_b_r!(3, h, "H", "dc");
set_b_r!(3, l, "L", "dd");
set_b_phl!(3, "de");
set_b_r!(3, a, "A", "df");

set_b_r!(4, b, "B", "e0");
set_b_r!(4, c, "C", "e1");
set_b_r!(4, d, "D", "e2");
set_b_r!(4, e, "E", "e3");
set_b_r!(4, h, "H", "e4");
set_b_r!(4, l, "L", "e5");
set_b_phl!(4, "e6");
set_b_r!(4, a, "A", "e7");

set_b_r!(5, b, "B", "e8");
set_b_r!(5, c, "C", "e9");
set_b_r!(5, d, "D", "ea");
set_b_r!(5, e, "E", "eb");
set_b_r!(5, h, "H", "ec");
set_b_r!(5, l, "L", "ed");
set_b_phl!(5, "ee");
set_b_r!(5, a, "A", "ef");

set_b_r!(6, b, "B", "f0");
set_b_r!(6, c, "C", "f1");
set_b_r!(6, d, "D", "f2");
set_b_r!(6, e, "E", "f3");
set_b_r!(6, h, "H", "f4");
set_b_r!(6, l, "L", "f5");
set_b_phl!(6, "f6");
set_b_r!(6, a, "A", "f7");

set_b_r!(7, b, "B", "f8");
set_b_r!(7, c, "C", "f9");
set_b_r!(7, d, "D", "fa");
set_b_r!(7, e, "E", "fb");
set_b_r!(7, h, "H", "fc");
set_b_r!(7, l, "L", "fd");
set_b_phl!(7, "fe");
set_b_r!(7, a, "A", "ff");

