use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

const COMMON: InstructionCommon = InstructionCommon {
    length: 2,
    cycles: 8,
    increment_pc: true,
};

pub struct LD_B_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_B_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_B_N, String> {
        Ok(LD_B_N {
            common: COMMON,
            n: memory.read_8(pos + 1)?,
        })
    }

    pub fn new_with_value(n: u8) -> LD_B_N {
        LD_B_N {
            common: COMMON,
            n,
        }
    }
}

impl Display for LD_B_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld b, 0x{:x}", self.n)
    }
}

impl BaseInstruction for LD_B_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x06, self.n]
    }
}

impl ExecutableInstruction<Z80> for LD_B_N {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.main.b = self.n;
        Ok(())
    }
}


use crate::generate_instruction_test;
generate_instruction_test!(LD_B_N, [0x07]);

