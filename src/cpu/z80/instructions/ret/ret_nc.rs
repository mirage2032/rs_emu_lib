use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::pop_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RET_NC {
    common: InstructionCommon,
}

impl RET_NC {
    pub fn new() -> RET_NC {
        RET_NC {
            common: InstructionCommon::new(1, 5, true),
        }
    }
}

impl Display for RET_NC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET NC")
    }
}

impl BaseInstruction for RET_NC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xd0]
    }
}

impl ExecutableInstruction<Z80> for RET_NC {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if !cpu.registers.gp.f.carry() {
            self.common = InstructionCommon::new(1, 11, false);
            cpu.registers.pc = pop_16!(memory, cpu.registers.sp);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("d0");
    test_instruction_parse!(RET_NC);
}
