use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_SP_IY {
    common: InstructionCommon,
}

impl LD_SP_IY {
    pub fn new() -> LD_SP_IY {
        LD_SP_IY {
            common: InstructionCommon::new(2, 10, true),
        }
    }
}

impl Display for LD_SP_IY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, IY",)
    }
}

impl BaseInstruction for LD_SP_IY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0xe9]
    }
}

impl ExecutableInstruction<Z80> for LD_SP_IY {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = cpu.registers.ix;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd e9");
    test_instruction_parse!(LD_SP_IY);
}
