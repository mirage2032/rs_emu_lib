use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct EX_PSP_IY {
    common: InstructionCommon,
}

impl EX_PSP_IY {
    pub fn new() -> EX_PSP_IY {
        EX_PSP_IY {
            common: InstructionCommon::new(2, 23, true),
        }
    }
}

impl Display for EX_PSP_IY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX (SP), IY")
    }
}

impl BaseInstruction for EX_PSP_IY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0xe3]
    }
}

impl ExecutableInstruction<Z80> for EX_PSP_IY {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_16(cpu.registers.sp)?;
        memory.write_16(cpu.registers.sp, cpu.registers.iy)?;
        cpu.registers.iy = val;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd","e3");
    test_instruction_parse!(EX_PSP_IY);
}
