use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::add::generics::add_r_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADD_A_N {
    common: InstructionCommon,
    n: u8,
}

impl ADD_A_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<ADD_A_N, MemoryReadError> {
        Ok(ADD_A_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> ADD_A_N {
        ADD_A_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for ADD_A_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A, 0x{:02X}", self.n)
    }
}

impl BaseInstruction for ADD_A_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xC6, self.n]
    }
}

impl ExecutableInstruction<Z80> for ADD_A_N {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        add_r_r_setf!(&mut cpu.registers.gp.a, self.n, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("c6");
    test_instruction_parse!(ADD_A_N, [0xce]);
}
