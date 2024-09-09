use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::instruction::push_16;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct PUSH_IX {
    common: InstructionCommon,
}

impl PUSH_IX {
    pub fn new() -> PUSH_IX {
        PUSH_IX {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for PUSH_IX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH IX")
    }
}

impl BaseInstruction for PUSH_IX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xe5]
    }
}

impl ExecutableInstruction<Z80> for PUSH_IX {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(cpu.registers.ix, memory, cpu.registers.sp);
        cpu.registers.r = cpu.registers.r.wrapping_add(1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("dd e5");
    test_instruction_parse!(PUSH_IX);
}
