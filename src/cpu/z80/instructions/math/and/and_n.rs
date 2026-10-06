use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
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
        write!(f, "AND 0x{:02X}", self.n)
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
        cpu.registers.gp.a = alu::and8(&mut cpu.registers.gp.f, cpu.registers.gp.a, self.n);
        Ok(())
    }
}
