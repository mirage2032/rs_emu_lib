use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_SP_IX {
    common: InstructionCommon,
}

impl LD_SP_IX {
    pub fn new() -> LD_SP_IX {
        LD_SP_IX {
            common: InstructionCommon::new(2, 10, true),
        }
    }
}

impl Display for LD_SP_IX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, IX",)
    }
}

impl BaseInstruction for LD_SP_IX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xf9]
    }
}

impl ExecutableInstruction<Z80> for LD_SP_IX {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = cpu.registers.ix;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd f9");
    test_instruction_parse!(LD_SP_IX);
}
