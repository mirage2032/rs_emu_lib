use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct DEC_DE {
    common: InstructionCommon,
}

impl DEC_DE {
    pub fn new() -> DEC_DE {
        DEC_DE {
            common: InstructionCommon::new(1, 6, true), //
        }
    }
}

impl Display for DEC_DE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC DE")
    }
}

impl BaseInstruction for DEC_DE {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x1b]
    }
}

impl ExecutableInstruction<Z80> for DEC_DE {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].de = cpu.registers.gp[0].de.wrapping_sub(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("1b.json");
    test_instruction_parse!(DEC_DE);
}
