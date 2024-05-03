use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct NOP {
    common: InstructionCommon,
}

impl NOP {
    pub fn new() -> NOP {
        NOP {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            }
        }
    }
}

impl Display for NOP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "NOP")
    }
}

impl BaseInstruction for NOP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x00]
    }
}

impl ExecutableInstruction<Z80> for NOP {
    fn runner(&self, _memory: &mut Memory, _cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_instruction_test;

    generate_instruction_test!(NOP);
}