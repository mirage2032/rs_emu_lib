use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct JP_PIX {
    common: InstructionCommon,
}

impl JP_PIX {
    pub fn new() -> JP_PIX{
        JP_PIX {
            common: InstructionCommon::new(2, 8, false),
        }
    }
}

impl Display for JP_PIX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP (IX)")
    }
}

impl BaseInstruction for JP_PIX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xDD, 0xE9]
    }
}

impl ExecutableInstruction<Z80> for JP_PIX {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = cpu.registers.ix;
        cpu.registers.r += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd", "e9");
    test_instruction_parse!(JP_PIX);
}
