use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct LD_A_C {
    common: InstructionCommon,
}

impl LD_A_C {
    pub fn new() -> LD_A_C {
        LD_A_C {
            common: *COMMON,
        }
    }
}

impl Display for LD_A_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD A, C",)
    }
}

impl BaseInstruction for LD_A_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x79]
    }
}

impl ExecutableInstruction<Z80> for LD_A_C {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].a = cpu.registers.gp[0].c;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("79.json");
    test_instruction_parse!(LD_A_C);
}
