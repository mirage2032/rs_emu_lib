use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct IN_A_N {
    common: InstructionCommon,
    n: u8,
}

impl IN_A_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<IN_A_N, MemoryReadError> {
        Ok(IN_A_N {
            common: InstructionCommon::new(2, 11, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> IN_A_N {
        IN_A_N {
            common: InstructionCommon::new(2, 11, true),
            n,
        }
    }
}

impl Display for IN_A_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IN A, 0x{:02X}", self.n)
    }
}

impl BaseInstruction for IN_A_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xDB, self.n]
    }
}

impl ExecutableInstruction<Z80> for IN_A_N {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.registers.gp.a = io.read(self.n)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("db");
    test_instruction_parse!(IN_A_N, [0xce]);
}
