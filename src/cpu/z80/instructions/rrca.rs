use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RRCA {
    common: InstructionCommon,
}

impl RRCA {
    pub fn new() -> RRCA {
        RRCA {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for RRCA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRCA")
    }
}

impl BaseInstruction for RRCA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0f]
    }
}

impl ExecutableInstruction<Z80> for RRCA {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp.a = alu::rrca(&mut cpu.registers.gp.f, cpu.registers.gp.a);
        Ok(())
    }
}
