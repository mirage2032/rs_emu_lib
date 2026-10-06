use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct OR_PHL {
    common: InstructionCommon,
}

impl OR_PHL {
    pub fn new() -> OR_PHL {
        OR_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for OR_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OR (HL)")
    }
}

impl BaseInstruction for OR_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xb6]
    }
}

impl ExecutableInstruction<Z80> for OR_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp.hl)?;
        cpu.registers.gp.a = alu::or8(&mut cpu.registers.gp.f, cpu.registers.gp.a, value);
        Ok(())
    }
}
