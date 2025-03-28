use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM1 {
    common: InstructionCommon,
}

impl IM1 {
    pub fn new() -> IM1 {
        IM1 {
            common: InstructionCommon::new(2, 8, true),
        }
    }
}

impl Display for IM1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM 1",)
    }
}

impl BaseInstruction for IM1 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x56]
    }
}

impl ExecutableInstruction<Z80> for IM1 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        // io.set_im(0); TODO: Implement IM1
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "56");
    test_instruction_parse!(IM1);
}
