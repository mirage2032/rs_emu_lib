use std::fmt;
use std::fmt::Display;


use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct EX_DE_HL {
    common: InstructionCommon,
}

impl EX_DE_HL {
    pub fn new() -> EX_DE_HL {
        EX_DE_HL {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for EX_DE_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX DE, HL")
    }
}

impl BaseInstruction for EX_DE_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xeb]
    }
}

impl ExecutableInstruction<Z80> for EX_DE_HL {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let temp = cpu.registers.gp.de;
        cpu.registers.gp.de = cpu.registers.gp.hl;
        cpu.registers.gp.hl = temp;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("eb");
    test_instruction_parse!(EX_DE_HL);
}
