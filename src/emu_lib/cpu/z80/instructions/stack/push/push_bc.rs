use crate::cpu::instruction::push_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct PUSH_BC {
    common: InstructionCommon,
}

impl PUSH_BC {
    pub fn new() -> PUSH_BC {
        PUSH_BC {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for PUSH_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH BC")
    }
}

impl BaseInstruction for PUSH_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xc5]
    }
}

impl ExecutableInstruction<Z80> for PUSH_BC {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(cpu.registers.gp[0].bc, memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("c5");
    test_instruction_parse!(PUSH_BC);
}
