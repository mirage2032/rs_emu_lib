use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct INC_IY {
    common: InstructionCommon,
}

impl INC_IY {
    pub fn new() -> INC_IY {
        INC_IY {
            common: InstructionCommon::new(2, 10, true),
        }
    }
}

impl Display for INC_IY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC IY",)
    }
}

impl BaseInstruction for INC_IY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0x23]
    }
}

impl ExecutableInstruction<Z80> for INC_IY {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.iy = cpu.registers.ix.wrapping_add(1);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd", "23");
    test_instruction_parse!(INC_IY);
}
