use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM2 {
    common: InstructionCommon,
}

impl IM2 {
    pub fn new() -> IM2 {
        IM2 {
            common: InstructionCommon::new(2, 8, true),
        }
    }
}

impl Display for IM2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM 2",)
    }
}

impl BaseInstruction for IM2 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x5E]
    }
}

impl ExecutableInstruction<Z80> for IM2 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        // io.set_im(0); TODO: Implement IM2
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "5e");
    test_instruction_parse!(IM2);
}
