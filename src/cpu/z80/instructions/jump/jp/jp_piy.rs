use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct JP_PIY {
    common: InstructionCommon,
}

impl JP_PIY {
    pub fn new() -> JP_PIY {
        JP_PIY {
            common: InstructionCommon::new(2, 8, false),
        }
    }
}

impl Display for JP_PIY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP (IY)")
    }
}

impl BaseInstruction for JP_PIY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFD, 0xE9]
    }
}

impl ExecutableInstruction<Z80> for JP_PIY {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = cpu.registers.iy;
        cpu.registers.r += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd", "e9");
    test_instruction_parse!(JP_PIY);
}
