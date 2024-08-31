use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct DEC_BC {
    common: InstructionCommon,
}

impl DEC_BC {
    pub fn new() -> DEC_BC {
        DEC_BC {
            common: InstructionCommon::new(1, 6, true), //
        }
    }
}

impl Display for DEC_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC BC")
    }
}

impl BaseInstruction for DEC_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0b]
    }
}

impl ExecutableInstruction<Z80> for DEC_BC {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].bc = cpu.registers.gp[0].bc.wrapping_sub(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("0b.json");
    test_instruction_parse!(DEC_BC);
}
