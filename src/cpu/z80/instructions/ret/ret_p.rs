use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::pop_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RET_P {
    common: InstructionCommon,
}

impl RET_P {
    pub fn new() -> RET_P {
        RET_P {
            common: InstructionCommon::new(1, 5, true),
        }
    }
}

impl Display for RET_P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET P")
    }
}

impl BaseInstruction for RET_P {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xf0]
    }
}

impl ExecutableInstruction<Z80> for RET_P {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if !cpu.registers.gp.f.sign() {
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

    test_z80!("f0");
    test_instruction_parse!(RET_P);
}
