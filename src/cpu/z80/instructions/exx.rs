use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;
use std::fmt;
use std::fmt::Display;
use std::mem::swap;

#[derive(Debug)]
pub struct EXX {
    common: InstructionCommon,
}

impl EXX {
    pub fn new() -> EXX {
        EXX {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for EXX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EXX")
    }
}

impl BaseInstruction for EXX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xd9]
    }
}

impl ExecutableInstruction<Z80> for EXX {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        swap(&mut cpu.registers.gp.bc, &mut cpu.registers.gp_alt.bc);
        swap(&mut cpu.registers.gp.de, &mut cpu.registers.gp_alt.de);
        swap(&mut cpu.registers.gp.hl, &mut cpu.registers.gp_alt.hl);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("d9");
    test_instruction_parse!(EXX);
}
