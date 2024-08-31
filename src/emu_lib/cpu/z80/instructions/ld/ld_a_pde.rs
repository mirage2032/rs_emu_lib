use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_A_PDE {
    common: InstructionCommon,
}

impl LD_A_PDE {
    pub fn new() -> LD_A_PDE {
        LD_A_PDE {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for LD_A_PDE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD A, (DE)",)
    }
}

impl BaseInstruction for LD_A_PDE {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x1a]
    }
}

impl ExecutableInstruction<Z80> for LD_A_PDE {
    fn runner(&self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].a = memory.read_8(cpu.registers.gp[0].de)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("1a.json");
    test_instruction_parse!(LD_A_PDE);
}
