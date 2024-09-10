use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct EX_AF_SAF {
    common: InstructionCommon,
}

impl EX_AF_SAF {
    pub fn new() -> EX_AF_SAF {
        EX_AF_SAF {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for EX_AF_SAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX AF, AF'")
    }
}

impl BaseInstruction for EX_AF_SAF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x08]
    }
}

impl ExecutableInstruction<Z80> for EX_AF_SAF {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let af = cpu.registers.gp.af;
        cpu.registers.gp.af = cpu.registers.gp_alt.af;
        cpu.registers.gp_alt.af = af;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("08");
    test_instruction_parse!(EX_AF_SAF);
}
