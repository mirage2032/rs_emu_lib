use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::or::or_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct OR_PHL {
    common: InstructionCommon,
}

impl OR_PHL {
    pub fn new() -> OR_PHL {
        OR_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for OR_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OR (HL)")
    }
}

impl BaseInstruction for OR_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xb6]
    }
}

impl ExecutableInstruction<Z80> for OR_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp.hl)?;
        or_r_setf!(cpu.registers.gp.a, value, cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("b6");
    test_instruction_parse!(OR_PHL);
}
