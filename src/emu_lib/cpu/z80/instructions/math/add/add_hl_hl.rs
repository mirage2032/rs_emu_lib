use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADD_HL_HL {
    common: InstructionCommon,
}

impl ADD_HL_HL {
    pub fn new() -> ADD_HL_HL {
        ADD_HL_HL {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for ADD_HL_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD HL, HL")
    }
}

impl BaseInstruction for ADD_HL_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x29]
    }
}

impl ExecutableInstruction<Z80> for ADD_HL_HL {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::add_rr_rr!(
            &mut cpu.registers.gp[0].hl,
            cpu.registers.gp[0].hl,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("29");
    test_instruction_parse!(ADD_HL_HL);
}
