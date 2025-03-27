use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::pop_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct POP_IY {
    common: InstructionCommon,
}

impl POP_IY {
    pub fn new() -> POP_IY {
        POP_IY {
            common: InstructionCommon::new(2, 14, true),
        }
    }
}

impl Display for POP_IY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP IY")
    }
}

impl BaseInstruction for POP_IY {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0xe1]
    }
}

impl ExecutableInstruction<Z80> for POP_IY {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let result = pop_16!(memory, cpu.registers.sp);
        cpu.registers.iy = result;
        cpu.registers.r = cpu.registers.r.wrapping_add(1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("fd e1");
    test_instruction_parse!(POP_IY);
}
