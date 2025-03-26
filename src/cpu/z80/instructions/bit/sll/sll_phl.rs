use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::bit::sll::generics::sll_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SLL_PHL {
    common: InstructionCommon,
}

impl SLL_PHL {
    pub fn new() -> SLL_PHL {
        SLL_PHL {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for SLL_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SLL (HL)")
    }
}

impl BaseInstruction for SLL_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xcb, 0x36]
    }
}

impl ExecutableInstruction<Z80> for SLL_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let mut value = memory.read_8(cpu.registers.gp.hl)?;
        sll_r_setf!(value, cpu.registers.gp.f);
        memory.write_8(cpu.registers.gp.hl, value)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("cb", "36");
    test_instruction_parse!(SLL_PHL);
}
