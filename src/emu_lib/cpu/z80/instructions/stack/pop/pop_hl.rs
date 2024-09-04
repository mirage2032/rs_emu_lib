use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::pop_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct POP_HL {
    common: InstructionCommon,
}

impl POP_HL {
    pub fn new() -> POP_HL {
        POP_HL {
            common: InstructionCommon::new(1, 10, true),
        }
    }
}

impl Display for POP_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP HL")
    }
}

impl BaseInstruction for POP_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xe1]
    }
}

impl ExecutableInstruction<Z80> for POP_HL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].hl = pop_16!(memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("e1");
    test_instruction_parse!(POP_HL);
}
