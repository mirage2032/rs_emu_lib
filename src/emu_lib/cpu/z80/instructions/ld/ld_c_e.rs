use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct LD_C_E {
    common: InstructionCommon,
}

impl LD_C_E {
    pub fn new() -> LD_C_E {
        LD_C_E {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for LD_C_E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD C, E",)
    }
}

impl BaseInstruction for LD_C_E {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x4b]
    }
}

impl ExecutableInstruction<Z80> for LD_C_E {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].c = cpu.registers.gp[0].e;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("4b");
    test_instruction_parse!(LD_C_E);
}
