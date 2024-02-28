use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::{Memory, ReadableMemory};

pub struct LD_C_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_C_N {
    pub fn new(memory: &Memory, pos: u16) -> Result<LD_C_N, String>{
        Ok(LD_C_N {
            common: InstructionCommon {
                length: 2,
                cycles: 7,
                increment_pc: true,
            },
            n: memory.read(pos + 1)?,
        })
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
}

impl ExecutableInstruction<Z80> for LD_C_N {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        cpu.registers.main.c = self.n;
        Ok(())
    }
}