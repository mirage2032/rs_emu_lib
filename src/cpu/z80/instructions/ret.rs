use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::instruction::pop_16;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RET {
    common: InstructionCommon,
}

impl RET {
    pub fn new() -> RET {
        RET {
            common: InstructionCommon::new(1, 10, false),
        }
    }
}

impl Display for RET {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET")
    }
}

impl BaseInstruction for RET {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xc9]
    }
}

impl ExecutableInstruction<Z80> for RET {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.pc = pop_16!(memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("c9");
    test_instruction_parse!(RET);
}
