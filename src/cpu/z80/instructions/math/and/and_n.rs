use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::and::and_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct AND_N {
    common: InstructionCommon,
    n: u8,
}

impl AND_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<AND_N, MemoryReadError> {
        Ok(AND_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> AND_N {
        AND_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for AND_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AND 0x{:02x}", self.n)
    }
}

impl BaseInstruction for AND_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xe6, self.n]
    }
}

impl ExecutableInstruction<Z80> for AND_N {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        and_r_setf!(cpu.registers.gp.a, self.n, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("e6");
    test_instruction_parse!(AND_N, [0xbf]);
}
