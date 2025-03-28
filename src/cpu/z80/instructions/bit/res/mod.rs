use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::res::fmt::Display;
use crate::cpu::z80::instructions::bit::res::generics::res_b_phl::res_b_phl;
use crate::cpu::z80::instructions::bit::res::generics::res_b_pixd::res_b_pixd;
use crate::cpu::z80::instructions::bit::res::generics::res_b_piyd::res_b_piyd;
use crate::cpu::z80::instructions::bit::res::generics::res_b_r::res_b_r;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::cpu::ExecutableInstruction;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;
use hex_literal::hex;
use std::fmt;

mod generics;

res_b_r!(0, b, "B", "80");
res_b_r!(0, c, "C", "81");
res_b_r!(0, d, "D", "82");
res_b_r!(0, e, "E", "83");
res_b_r!(0, h, "H", "84");
res_b_r!(0, l, "L", "85");
res_b_phl!(0, "86");
res_b_r!(0, a, "A", "87");

res_b_r!(1, b, "B", "88");
res_b_r!(1, c, "C", "89");
res_b_r!(1, d, "D", "8a");
res_b_r!(1, e, "E", "8b");
res_b_r!(1, h, "H", "8c");
res_b_r!(1, l, "L", "8d");
res_b_phl!(1, "8e");
res_b_r!(1, a, "A", "8f");

res_b_r!(2, b, "B", "90");
res_b_r!(2, c, "C", "91");
res_b_r!(2, d, "D", "92");
res_b_r!(2, e, "E", "93");
res_b_r!(2, h, "H", "94");
res_b_r!(2, l, "L", "95");
res_b_phl!(2, "96");
res_b_r!(2, a, "A", "97");

res_b_r!(3, b, "B", "98");
res_b_r!(3, c, "C", "99");
res_b_r!(3, d, "D", "9a");
res_b_r!(3, e, "E", "9b");
res_b_r!(3, h, "H", "9c");
res_b_r!(3, l, "L", "9d");
res_b_phl!(3, "9e");
res_b_r!(3, a, "A", "9f");

res_b_r!(4, b, "B", "a0");
res_b_r!(4, c, "C", "a1");
res_b_r!(4, d, "D", "a2");
res_b_r!(4, e, "E", "a3");
res_b_r!(4, h, "H", "a4");
res_b_r!(4, l, "L", "a5");
res_b_phl!(4, "a6");
res_b_r!(4, a, "A", "a7");

res_b_r!(5, b, "B", "a8");
res_b_r!(5, c, "C", "a9");
res_b_r!(5, d, "D", "aa");
res_b_r!(5, e, "E", "ab");
res_b_r!(5, h, "H", "ac");
res_b_r!(5, l, "L", "ad");
res_b_phl!(5, "ae");
res_b_r!(5, a, "A", "af");

res_b_r!(6, b, "B", "b0");
res_b_r!(6, c, "C", "b1");
res_b_r!(6, d, "D", "b2");
res_b_r!(6, e, "E", "b3");
res_b_r!(6, h, "H", "b4");
res_b_r!(6, l, "L", "b5");
res_b_phl!(6, "b6");
res_b_r!(6, a, "A", "b7");

res_b_r!(7, b, "B", "b8");
res_b_r!(7, c, "C", "b9");
res_b_r!(7, d, "D", "ba");
res_b_r!(7, e, "E", "bb");
res_b_r!(7, h, "H", "bc");
res_b_r!(7, l, "L", "bd");
res_b_phl!(7, "be");
res_b_r!(7, a, "A", "bf");

res_b_pixd!(0, "86");
res_b_pixd!(1, "8e");
res_b_pixd!(2, "96");
res_b_pixd!(3, "9e");
res_b_pixd!(4, "a6");
res_b_pixd!(5, "ae");
res_b_pixd!(6, "b6");
res_b_pixd!(7, "be");

res_b_piyd!(0, "86");
res_b_piyd!(1, "8e");
res_b_piyd!(2, "96");
res_b_piyd!(3, "9e");
res_b_piyd!(4, "a6");
res_b_piyd!(5, "ae");
res_b_piyd!(6, "b6");
res_b_piyd!(7, "be");
