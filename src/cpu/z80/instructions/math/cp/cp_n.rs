use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct CP_N {
    common: InstructionCommon,
    n: u8,
}

impl CP_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<CP_N, MemoryReadError> {
        Ok(Self::new_with_value(memory.read_8(pos.wrapping_add(1))?))
    }

    pub fn new_with_value(n: u8) -> CP_N {
        CP_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for CP_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CP 0x{:02X}", self.n)
    }
}

impl BaseInstruction for CP_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFE, self.n]
    }
}

impl ExecutableInstruction<Z80> for CP_N {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        alu::cp8(&mut cpu.registers.gp.f, cpu.registers.gp.a, self.n);
        Ok(())
    }
}
