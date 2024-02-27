use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;

pub struct LD_PBC_A {
    common: InstructionCommon,
}

impl LD_PBC_A {
    pub fn new(memory: &Memory, pos: u16) -> LD_PBC_A {
        LD_PBC_A {
            common: InstructionCommon {
                length: 1,
                cycles: 7,
                increment_pc: true,
            },
        }
    }
}

impl Display for LD_PBC_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld (bc), a")
    }
}

impl BaseInstruction for LD_PBC_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
}

impl ExecutableInstruction<Z80> for LD_PBC_A {
    fn runner(&self, memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        let location = cpu.registers.main.bc;
        memory.write8(location, cpu.registers.main.a)?;
        Ok(())
    }
}