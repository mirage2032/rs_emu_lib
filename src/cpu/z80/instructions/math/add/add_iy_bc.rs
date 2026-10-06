use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ADD_IY_BC {
    common: InstructionCommon,
}

impl ADD_IY_BC {
    pub fn new() -> ADD_IY_BC {
        ADD_IY_BC {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for ADD_IY_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD IY, BC")
    }
}

impl BaseInstruction for ADD_IY_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0x09]
    }
}

impl ExecutableInstruction<Z80> for ADD_IY_BC {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let (a, b) = (cpu.registers.iy, cpu.registers.gp.bc);
        cpu.registers.iy = alu::add16(&mut cpu.registers.gp.f, a, b);
        cpu.registers.inc_r();
        Ok(())
    }
}
