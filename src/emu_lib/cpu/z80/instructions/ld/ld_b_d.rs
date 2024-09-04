use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct LD_B_D {
    common: InstructionCommon,
}

impl LD_B_D {
    pub fn new() -> LD_B_D {
        LD_B_D {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for LD_B_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD B, D",)
    }
}

impl BaseInstruction for LD_B_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x42]
    }
}

impl ExecutableInstruction<Z80> for LD_B_D {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].b = cpu.registers.gp[0].d;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("42");
    test_instruction_parse!(LD_B_D);
}
