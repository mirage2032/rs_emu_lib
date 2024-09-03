use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct DEC_L {
    common: InstructionCommon,
}

impl DEC_L {
    pub fn new() -> DEC_L {
        DEC_L { common: *COMMON }
    }
}

impl Display for DEC_L {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC L",)
    }
}

impl BaseInstruction for DEC_L {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x2d]
    }
}

impl ExecutableInstruction<Z80> for DEC_L {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::dec_r!(&mut gp.l, &mut gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("2d.json");
    test_instruction_parse!(DEC_L);
}
