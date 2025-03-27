use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::add::generics::add_rr_rr_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ADD_IY_DE {
    common: InstructionCommon,
}

impl ADD_IY_DE {
    pub fn new() -> ADD_IY_DE {
        ADD_IY_DE {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for ADD_IY_DE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD IY, DE")
    }
}

impl BaseInstruction for ADD_IY_DE {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0x19]
    }
}

impl ExecutableInstruction<Z80> for ADD_IY_DE {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        add_rr_rr_setf!(cpu.registers.iy, cpu.registers.gp.de, cpu.registers.gp.f);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd 19");
    test_instruction_parse!(ADD_IY_DE);
}
