use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct JP_PHL {
    common: InstructionCommon,
}

impl JP_PHL {
    pub fn new() -> JP_PHL{
        JP_PHL {
            common: InstructionCommon::new(1, 4, false),
        }
    }
}

impl Display for JP_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP (HL)")
    }
}

impl BaseInstruction for JP_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xE9]
    }
}

impl ExecutableInstruction<Z80> for JP_PHL {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = cpu.registers.gp.hl;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("e9");
    test_instruction_parse!(JP_PHL);
}
