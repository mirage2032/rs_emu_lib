use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::sbc::generics::sbc_rr_rr;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct SBC_HL_HL {
    common: InstructionCommon,
}

impl SBC_HL_HL {
    pub fn new() -> SBC_HL_HL {
        SBC_HL_HL {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for SBC_HL_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SBC HL, HL")
    }
}

impl BaseInstruction for SBC_HL_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, 0x62]
    }
}

impl ExecutableInstruction<Z80> for SBC_HL_HL {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        sbc_rr_rr!(cpu.registers.gp.hl, cpu.registers.gp.hl, cpu.registers.gp.f);
        cpu.registers.r = cpu.registers.r.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 62");
    test_instruction_parse!(SBC_HL_HL);
}
