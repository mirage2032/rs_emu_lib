use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct OUT_N_A {
    common: InstructionCommon,
    n: u8,
}

impl OUT_N_A {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<OUT_N_A, MemoryReadError> {
        Ok(OUT_N_A {
            common: InstructionCommon::new(2, 11, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> OUT_N_A {
        OUT_N_A {
            common: InstructionCommon::new(2, 11, true),
            n,
        }
    }
}

impl Display for OUT_N_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OUT 0x{:02x}, A", self.n)
    }
}

impl BaseInstruction for OUT_N_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xD3, self.n]
    }
}

impl ExecutableInstruction<Z80> for OUT_N_A {
    fn execute(&mut self, _memory: &mut Memory, _cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        io.write(self.n, _cpu.registers.gp.a)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("d3");
    test_instruction_parse!(OUT_N_A, [0xce]);
}
