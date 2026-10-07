use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RETI {
    common: InstructionCommon,
}

impl RETI {
    pub fn new() -> RETI {
        RETI {
            common: InstructionCommon::new(2, 14, false),
        }
    }
}

impl Display for RETI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RETI",)
    }
}

impl BaseInstruction for RETI {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x4d]
    }
}

impl ExecutableInstruction<Z80> for RETI {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = pop_16!(memory, cpu.registers.sp);
        cpu.interrupts.iff1 = cpu.interrupts.iff2;
        Ok(())
    }
}
