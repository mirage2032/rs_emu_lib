use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RETN {
    common: InstructionCommon,
}

impl RETN {
    pub fn new() -> RETN {
        RETN {
            common: InstructionCommon::new(2, 14, false),
        }
    }
}

impl Display for RETN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RETN",)
    }
}

impl BaseInstruction for RETN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x45]
    }
}

impl ExecutableInstruction<Z80> for RETN {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = pop_16!(memory, cpu.registers.sp);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        io.disable_int();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "45");
    test_instruction_parse!(RETN);
}
