use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::sub::sub_r;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SUB_N {
    common: InstructionCommon,
    n: u8,
}

impl SUB_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SUB_N, String> {
        Ok(SUB_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> SUB_N {
        SUB_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for SUB_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUB 0x{:02x}", self.n)
    }
}

impl BaseInstruction for SUB_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xD6, self.n]
    }
}

impl ExecutableInstruction<Z80> for SUB_N {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        sub_r!(cpu.registers.gp.a, self.n, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("d6");
    test_instruction_parse!(SUB_N, [0xbf]);
}
