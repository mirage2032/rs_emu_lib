use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::sbc::generics::sbc_r_r;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct SBC_A_PHL {
    common: InstructionCommon,
}

impl SBC_A_PHL {
    pub fn new() -> SBC_A_PHL {
        SBC_A_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for SBC_A_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SBC A, (HL)")
    }
}

impl BaseInstruction for SBC_A_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x9E]
    }
}

impl ExecutableInstruction<Z80> for SBC_A_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.gp.hl)?;
        sbc_r_r!(cpu.registers.gp.a, val, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("9e");
    test_instruction_parse!(SBC_A_PHL);
}
