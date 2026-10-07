use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct DEC_PHL {
    common: InstructionCommon,
}

impl DEC_PHL {
    pub fn new() -> DEC_PHL {
        DEC_PHL {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for DEC_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC (HL)")
    }
}

impl BaseInstruction for DEC_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x35]
    }
}

impl ExecutableInstruction<Z80> for DEC_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp.hl)?;
        let result = alu::dec8(&mut cpu.registers.gp.f, value);
        memory.write_8(cpu.registers.gp.hl, result)?;
        Ok(())
    }
}
