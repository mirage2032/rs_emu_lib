use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct DEC_IY {
    common: InstructionCommon,
}

impl DEC_IY {
    pub fn new() -> DEC_IY {
        DEC_IY {
            common: InstructionCommon::new(2, 10, true),
        }
    }
}

impl Display for DEC_IY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC IY",)
    }
}

impl BaseInstruction for DEC_IY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd,0x2b]
    }
}

impl ExecutableInstruction<Z80> for DEC_IY {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.iy = cpu.registers.iy.wrapping_sub(1);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd", "2b");
    test_instruction_parse!(DEC_IY);
}
