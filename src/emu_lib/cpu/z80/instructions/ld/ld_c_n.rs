use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::{Memory, ReadableMemory};

const COMMON: InstructionCommon = InstructionCommon {
    length: 2,
    cycles: 8,
    increment_pc: true,
};

pub struct LD_C_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_C_N {
    pub fn new<T: ReadableMemory>(memory: &T, pos: u16) -> Result<LD_C_N, String> {
        Ok(LD_C_N {
            common: COMMON,
            n: memory.read_8(pos + 1)?,
        })
    }
    pub fn new_with_value(n: u8) -> LD_C_N {
        LD_C_N {
            common: COMMON,
            n,
        }
    }
}

impl Display for LD_C_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld c, n")
    }
}

impl BaseInstruction for LD_C_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0e, self.n]
    }
}

impl ExecutableInstruction<Z80> for LD_C_N {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        cpu.registers.main.c = self.n;
        Ok(())
    }
}