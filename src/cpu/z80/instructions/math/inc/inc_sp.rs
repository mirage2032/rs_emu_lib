use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct INC_SP {
    common: InstructionCommon,
}

impl INC_SP {
    pub fn new() -> INC_SP {
        INC_SP {
            common: InstructionCommon::new(1, 6, true),
        }
    }
}

impl Display for INC_SP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC SP",)
    }
}

impl BaseInstruction for INC_SP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x33]
    }
}

impl ExecutableInstruction<Z80> for INC_SP {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("33");
    test_instruction_parse!(INC_SP);
}
