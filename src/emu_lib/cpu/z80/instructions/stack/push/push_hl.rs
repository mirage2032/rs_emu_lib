use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::push_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct PUSH_HL {
    common: InstructionCommon,
}

impl PUSH_HL {
    pub fn new() -> PUSH_HL {
        PUSH_HL {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for PUSH_HL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH HL")
    }
}

impl BaseInstruction for PUSH_HL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xe5]
    }
}

impl ExecutableInstruction<Z80> for PUSH_HL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(cpu.registers.gp[0].hl, memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("e5");
    test_instruction_parse!(PUSH_HL);
}
