use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 6, true));

#[derive(Debug)]
pub struct INC_DE {
    common: InstructionCommon,
}

impl INC_DE {
    pub fn new() -> INC_DE {
        INC_DE { common: *COMMON }
    }
}

impl Display for INC_DE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC DE",)
    }
}

impl BaseInstruction for INC_DE {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x13]
    }
}

impl ExecutableInstruction<Z80> for INC_DE {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].de = cpu.registers.gp[0].de.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("13.json");
    test_instruction_parse!(INC_DE);
}
