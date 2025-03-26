use crate::memory::MemoryDevice;
use crate::cpu::instruction::InstructionCommon;
use crate::cpu::z80::instructions::bit::bit::fmt::Display;
use crate::cpu::z80::instructions::bit::bit::generics::bit_b_r::bit_b_r;
use crate::cpu::z80::instructions::bit::bit::generics::bit_b_phl::bit_b_phl;
use crate::cpu::z80::instructions::bit::bit::generics::bit_b_r_setf;
use crate::cpu::z80::Z80;
use crate::cpu::BaseInstruction;
use crate::cpu::ExecutableInstruction;
use crate::io::IO;
use crate::memory::Memory;
use hex_literal::hex;
use std::fmt;

mod generics;

bit_b_r!(0, b, "B", "40");
bit_b_r!(0, c, "C", "41");
bit_b_r!(0, d, "D", "42");
bit_b_r!(0, e, "E", "43");
bit_b_r!(0, h, "H", "44");
bit_b_r!(0, l, "L", "45");
bit_b_phl!(0, "46");
bit_b_r!(0, a, "A", "47");

bit_b_r!(1, b, "B", "48");
bit_b_r!(1, c, "C", "49");
bit_b_r!(1, d, "D", "4a");
bit_b_r!(1, e, "E", "4b");
bit_b_r!(1, h, "H", "4c");
bit_b_r!(1, l, "L", "4d");
bit_b_phl!(1, "4e");
bit_b_r!(1, a, "A", "4f");

bit_b_r!(2, b, "B", "50");
bit_b_r!(2, c, "C", "51");
bit_b_r!(2, d, "D", "52");
bit_b_r!(2, e, "E", "53");
bit_b_r!(2, h, "H", "54");
bit_b_r!(2, l, "L", "55");
bit_b_phl!(2, "56");
bit_b_r!(2, a, "A", "57");

bit_b_r!(3, b, "B", "58");
bit_b_r!(3, c, "C", "59");
bit_b_r!(3, d, "D", "5a");
bit_b_r!(3, e, "E", "5b");
bit_b_r!(3, h, "H", "5c");
bit_b_r!(3, l, "L", "5d");
bit_b_phl!(3, "5e");
bit_b_r!(3, a, "A", "5f");

bit_b_r!(4, b, "B", "60");
bit_b_r!(4, c, "C", "61");
bit_b_r!(4, d, "D", "62");
bit_b_r!(4, e, "E", "63");
bit_b_r!(4, h, "H", "64");
bit_b_r!(4, l, "L", "65");
bit_b_phl!(4, "66");
bit_b_r!(4, a, "A", "67");

bit_b_r!(5, b, "B", "68");
bit_b_r!(5, c, "C", "69");
bit_b_r!(5, d, "D", "6a");
bit_b_r!(5, e, "E", "6b");
bit_b_r!(5, h, "H", "6c");
bit_b_r!(5, l, "L", "6d");
bit_b_phl!(5, "6e");
bit_b_r!(5, a, "A", "6f");

bit_b_r!(6, b, "B", "70");
bit_b_r!(6, c, "C", "71");
bit_b_r!(6, d, "D", "72");
bit_b_r!(6, e, "E", "73");
bit_b_r!(6, h, "H", "74");
bit_b_r!(6, l, "L", "75");
bit_b_phl!(6, "76");
bit_b_r!(6, a, "A", "77");

bit_b_r!(7, b, "B", "78");
bit_b_r!(7, c, "C", "79");
bit_b_r!(7, d, "D", "7a");
bit_b_r!(7, e, "E", "7b");
bit_b_r!(7, h, "H", "7c");
bit_b_r!(7, l, "L", "7d");
bit_b_phl!(7, "7e");
bit_b_r!(7, a, "A", "7f");

