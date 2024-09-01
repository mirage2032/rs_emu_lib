use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
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
        write!(f, "ADD HL, BC")
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
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::add_rr_rr!(
            &mut cpu.registers.gp[0].hl,
            cpu.registers.gp[0].bc,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("09.json");
    test_instruction_parse!(ADD_HL_BC);
}
