use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::cp::cp_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct CP_N {
    common: InstructionCommon,
    n: u8,
}

impl CP_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<CP_N, String> {
        Ok(CP_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
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
        write!(f, "CP 0x{:02x}", self.n)
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
        cp_r_setf!(cpu.registers.gp.a, self.n, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fe");
    test_instruction_parse!(CP_N, [0xbf]);
}
