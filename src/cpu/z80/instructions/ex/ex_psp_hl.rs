use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct EX_PSP_HL {
    common: InstructionCommon,
}

impl EX_PSP_HL {
    pub fn new() -> EX_PSP_HL {
        EX_PSP_HL {
            common: InstructionCommon::new(1, 19, true),
        }
    }
}

impl Display for EX_PSP_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX (SP), HL")
    }
}

impl BaseInstruction for EX_PSP_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xe3]
    }
}

impl ExecutableInstruction<Z80> for EX_PSP_HL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_16(cpu.registers.sp)?;
        memory.write_16(cpu.registers.sp, cpu.registers.gp.hl)?;
        cpu.registers.gp.hl = val;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("e3");
    test_instruction_parse!(EX_PSP_HL);
}