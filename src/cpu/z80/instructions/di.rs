use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct DI {
    common: InstructionCommon,
}

impl DI {
    pub fn new() -> DI {
        DI {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for DI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DI")
    }
}

impl BaseInstruction for DI {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xf3]
    }
}

impl ExecutableInstruction<Z80> for DI {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.interrupts.iff1 = false;
        cpu.interrupts.iff2 = false;
        Ok(())
    }
}
