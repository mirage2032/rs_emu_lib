use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::{Memory, ReadableMemory};
use crate::io::IO;

pub struct LD_A_PBC {
    common: InstructionCommon,
}

impl LD_A_PBC {
    pub fn new() -> LD_A_PBC {
        LD_A_PBC {
            common: InstructionCommon {
                length: 1,
                cycles: 7,
                increment_pc: true,
            },
        }
    }
}

impl Display for LD_A_PBC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld a, (bc)", )
    }
}

impl BaseInstruction for LD_A_PBC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0a]
    }
}

impl ExecutableInstruction<Z80> for LD_A_PBC {
    fn runner(&self, memory: &mut Memory, cpu: &mut Z80,_:&mut IO) -> Result<(), String> {
        cpu.registers.main.a = memory.read_8(cpu.registers.main.bc)?;
        Ok(())
    }
}