use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::bit::sra::generics::sra_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SRA_PHL {
    common: InstructionCommon,
}

impl SRA_PHL {
    pub fn new() -> SRA_PHL {
        SRA_PHL {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for SRA_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRA (HL)")
    }
}

impl BaseInstruction for SRA_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xcb, 0x2e]
    }
}

impl ExecutableInstruction<Z80> for SRA_PHL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let mut value = memory.read_8(cpu.registers.gp.hl)?;
        sra_r_setf!(value, cpu.registers.gp.f);
        memory.write_8(cpu.registers.gp.hl, value)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("cb", "2e");
    test_instruction_parse!(SRA_PHL);
}
