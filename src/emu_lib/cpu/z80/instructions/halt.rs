use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::io::IO;

pub struct Halt {
    common: InstructionCommon,
}

impl Halt {
    pub fn new() -> Halt {
        Halt {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for Halt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "halt")
    }
}

impl BaseInstruction for Halt {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x76]
    }
}

impl ExecutableInstruction<Z80> for Halt {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.halted = true;
        Ok(())
    }
}