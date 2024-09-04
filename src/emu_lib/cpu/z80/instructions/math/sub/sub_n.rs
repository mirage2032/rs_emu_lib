use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;
use crate::cpu::z80::instructions::math::sub::sub_r;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(2, 7, true));

#[derive(Debug)]
pub struct SUB_N {
    common: InstructionCommon,
    n: u8,
}

impl SUB_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SUB_N, String> {
        Ok(SUB_N {
            common: *COMMON,
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> SUB_N {
        SUB_N {
            common: *COMMON,
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
        sub_r!(cpu.registers.gp[0].a, self.n, cpu.registers.gp[0].f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;
    test_z80!("d6");
    test_instruction_parse!(SUB_N, [0xbf]);
}
