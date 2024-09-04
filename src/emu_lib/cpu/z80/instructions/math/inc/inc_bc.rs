use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct INC_BC {
    common: InstructionCommon,
}

impl INC_BC {
    pub fn new() -> INC_BC {
        INC_BC {
            common: InstructionCommon::new(1, 6, true),
        }
    }
}

impl Display for INC_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC BC")
    }
}

impl BaseInstruction for INC_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x03]
    }
}

impl ExecutableInstruction<Z80> for INC_BC {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].bc = cpu.registers.gp[0].bc.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("03");
    test_instruction_parse!(INC_BC);
}
