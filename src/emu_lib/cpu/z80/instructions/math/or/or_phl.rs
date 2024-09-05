use std::fmt;
use std::fmt::Display;

use crate::cpu::z80::instructions::math::or::or_r_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

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
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp[0].hl)?;
        or_r_setf!(cpu.registers.gp[0].a, value, cpu.registers.gp[0].f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("b6");
    test_instruction_parse!(OR_PHL);
}
