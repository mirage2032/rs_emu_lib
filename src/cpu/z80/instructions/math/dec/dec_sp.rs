use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct DEC_SP {
    common: InstructionCommon,
}

impl DEC_SP {
    pub fn new() -> DEC_SP {
        DEC_SP {
            common: InstructionCommon::new(1, 6, true),
        }
    }
}

impl Display for DEC_SP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC SP",)
    }
}

impl BaseInstruction for DEC_SP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x3b]
    }
}

impl ExecutableInstruction<Z80> for DEC_SP {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("3b");
    test_instruction_parse!(DEC_SP);
}
