use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct INC_D {
    common: InstructionCommon,
}

impl INC_D {
    pub fn new() -> INC_D {
        INC_D { common: *COMMON }
    }
}

impl Display for INC_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC D",)
    }
}

impl BaseInstruction for INC_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x14]
    }
}

impl ExecutableInstruction<Z80> for INC_D {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::inc_r!(&mut gp.d, gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("14.json");
    test_instruction_parse!(INC_D);
}
