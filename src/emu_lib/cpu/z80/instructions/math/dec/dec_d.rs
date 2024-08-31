use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct DEC_D {
    common: InstructionCommon,
}

impl DEC_D {
    pub fn new() -> DEC_D {
        DEC_D { common: *COMMON }
    }
}

impl Display for DEC_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC D",)
    }
}

impl BaseInstruction for DEC_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x15]
    }
}

impl ExecutableInstruction<Z80> for DEC_D {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::dec_r!(&mut gp.d, &mut gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("15.json");
    test_instruction_parse!(DEC_D);
}
