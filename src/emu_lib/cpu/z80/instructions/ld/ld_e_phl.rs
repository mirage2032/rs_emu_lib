use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_E_PHL {
    common: InstructionCommon,
}

impl LD_E_PHL {
    pub fn new() -> LD_E_PHL {
        LD_E_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for LD_E_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD E, (HL)",)
    }
}

impl BaseInstruction for LD_E_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x5e]
    }
}

impl ExecutableInstruction<Z80> for LD_E_PHL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].e = memory.read_8(cpu.registers.gp[0].hl)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("5e.json");
    test_instruction_parse!(LD_E_PHL);
}
