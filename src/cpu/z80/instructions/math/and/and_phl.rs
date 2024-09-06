use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::and::and_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct AND_PHL {
    common: InstructionCommon,
}

impl AND_PHL {
    pub fn new() -> AND_PHL {
        AND_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for AND_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AND (HL)")
    }
}

impl BaseInstruction for AND_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xa6]
    }
}

impl ExecutableInstruction<Z80> for AND_PHL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp[0].hl)?;
        and_r_setf!(cpu.registers.gp[0].a, value, cpu.registers.gp[0].f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("a6");
    test_instruction_parse!(AND_PHL);
}
