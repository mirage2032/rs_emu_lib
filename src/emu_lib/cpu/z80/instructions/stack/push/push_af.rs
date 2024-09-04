use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::push_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct PUSH_AF {
    common: InstructionCommon,
}

impl PUSH_AF {
    pub fn new() -> PUSH_AF {
        PUSH_AF {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for PUSH_AF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH AF")
    }
}

impl BaseInstruction for PUSH_AF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xf5]
    }
}

impl ExecutableInstruction<Z80> for PUSH_AF {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(cpu.registers.gp[0].af, memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("f5");
    test_instruction_parse!(PUSH_AF);
}
