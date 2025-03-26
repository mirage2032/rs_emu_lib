use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct INC_IX {
    common: InstructionCommon,
}

impl INC_IX {
    pub fn new() -> INC_IX {
        INC_IX {
            common: InstructionCommon::new(2, 10, true),
        }
    }
}

impl Display for INC_IX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC IX",)
    }
}

impl BaseInstruction for INC_IX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x23]
    }
}

impl ExecutableInstruction<Z80> for INC_IX {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.ix = cpu.registers.ix.wrapping_add(1);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd", "23");
    test_instruction_parse!(INC_IX);
}
