use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::registers::Flags;
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::{Memory, ReadableMemory};

pub struct LD_B_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_B_N {
    pub fn new(memory: &Memory, pos: u16) -> Result<LD_B_N,String> {
        Ok(LD_B_N {
            common: InstructionCommon {
                length: 2,
                cycles: 7,
                increment_pc: true,
            },
            n: memory.read(pos + 1)?,
        })
    }
}

impl Display for LD_B_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld b, {:x}", self.n)
    }
}

impl BaseInstruction for LD_B_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
}

impl ExecutableInstruction<Z80> for LD_B_N {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        cpu.registers.main.b = self.n;
        Ok(())
    }
}