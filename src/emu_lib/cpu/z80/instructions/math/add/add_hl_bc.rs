use std::fmt;
use std::fmt::Display;

use crate::add_rr_rr;
use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;

pub struct ADD_HL_BC {
    common: InstructionCommon,
}

impl ADD_HL_BC {
    pub fn new() -> ADD_HL_BC {
        ADD_HL_BC {
            common: InstructionCommon {
                length: 3,
                cycles: 10,
                increment_pc: true,
            },
        }
    }
}

impl Display for ADD_HL_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "add hl, bc")
    }
}

impl BaseInstruction for ADD_HL_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
}

impl ExecutableInstruction<Z80> for ADD_HL_BC {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        add_rr_rr!(&mut cpu.registers.main.hl, cpu.registers.main.bc, cpu.registers.main.f);
        Ok(())
    }
}