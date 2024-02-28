use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;

pub struct EX_AF_SAF {
    common: InstructionCommon,
}

impl EX_AF_SAF {
    pub fn new() -> EX_AF_SAF {
        EX_AF_SAF {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for EX_AF_SAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ex af, af'")
    }
}

impl BaseInstruction for EX_AF_SAF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
}

impl ExecutableInstruction<Z80> for EX_AF_SAF {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        let af = cpu.registers.main.af;
        cpu.registers.main.af = cpu.registers.shadow.af;
        cpu.registers.shadow.af = af;
        Ok(())
    }
}