use crate::cpu::instruction::{pop_16, push_16};
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct POP_BC {
    common: InstructionCommon,
}

impl POP_BC {
    pub fn new() -> POP_BC {
        POP_BC {
            common: InstructionCommon::new(1, 10, true),
        }
    }
}

impl Display for POP_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP BC")
    }
}

impl BaseInstruction for POP_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xc1]
    }
}

impl ExecutableInstruction<Z80> for POP_BC {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].bc = pop_16!(memory, cpu.registers.sp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("c1.json");
    test_instruction_parse!(POP_BC);
}
