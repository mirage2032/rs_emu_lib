use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct DEC_E {
    common: InstructionCommon,
}

impl DEC_E {
    pub fn new() -> DEC_E {
        DEC_E { common: *COMMON }
    }
}

impl Display for DEC_E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC E",)
    }
}

impl BaseInstruction for DEC_E {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x1d]
    }
}

impl ExecutableInstruction<Z80> for DEC_E {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::dec_r!(&mut gp.e, &mut gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("1d.json");
    test_instruction_parse!(DEC_E);
}
