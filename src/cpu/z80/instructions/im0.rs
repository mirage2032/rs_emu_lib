use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM0 {
    common: InstructionCommon,
}

impl IM0 {
    pub fn new() -> IM0 {
        IM0 {
            common: InstructionCommon::new(2, 8, false),
        }
    }
}

impl Display for IM0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM10",)
    }
}

impl BaseInstruction for IM0 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x46]
    }
}

impl ExecutableInstruction<Z80> for IM0 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        // io.set_im(0);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "46");
    test_instruction_parse!(IM0);
}
