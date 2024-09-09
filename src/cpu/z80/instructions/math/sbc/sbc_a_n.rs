use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SBC_A_N {
    common: InstructionCommon,
    n: u8,
}

impl SBC_A_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SBC_A_N, String> {
        Ok(SBC_A_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> SBC_A_N {
        SBC_A_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for SBC_A_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUB 0x{:02x}", self.n)
    }
}

impl BaseInstruction for SBC_A_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xde, self.n]
    }
}

impl ExecutableInstruction<Z80> for SBC_A_N {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        sbc_r_r!(cpu.registers.gp.a, self.n, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("de");
    test_instruction_parse!(SBC_A_N, [0xbf]);
}
