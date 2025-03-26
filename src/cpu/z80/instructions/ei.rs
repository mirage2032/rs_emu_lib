use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct EI {
    common: InstructionCommon,
}

impl EI {
    pub fn new() -> EI {
        EI {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for EI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EI")
    }
}

impl BaseInstruction for EI {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfb]
    }
}

impl ExecutableInstruction<Z80> for EI {
    fn execute(&mut self, _memory: &mut Memory, _cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        io.iff1 = true;
        io.iff2 = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("fb");
    test_instruction_parse!(EI);
}
