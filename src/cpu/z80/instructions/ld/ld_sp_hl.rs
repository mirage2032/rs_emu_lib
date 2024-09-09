use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_SP_HL {
    common: InstructionCommon,
}

impl LD_SP_HL {
    pub fn new() -> LD_SP_HL {
        LD_SP_HL {
            common: InstructionCommon::new(1, 6, true),
        }
    }
}

impl Display for LD_SP_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, HL",)
    }
}

impl BaseInstruction for LD_SP_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xf9]
    }
}

impl ExecutableInstruction<Z80> for LD_SP_HL {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = cpu.registers.gp.hl;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("f9");
    test_instruction_parse!(LD_SP_HL);
}
