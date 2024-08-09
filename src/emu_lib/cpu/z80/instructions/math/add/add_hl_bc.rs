use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct ADD_HL_BC {
    common: InstructionCommon,
}

impl ADD_HL_BC {
    pub fn new() -> ADD_HL_BC {
        ADD_HL_BC {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for ADD_HL_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD hl, bc")
    }
}

impl BaseInstruction for ADD_HL_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x09]
    }
}

impl ExecutableInstruction<Z80> for ADD_HL_BC {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::add_rr_rr!(
            &mut cpu.registers.main.hl,
            cpu.registers.main.bc,
            cpu.registers.main.f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;

    test_instruction_parse!(ADD_HL_BC);
}
