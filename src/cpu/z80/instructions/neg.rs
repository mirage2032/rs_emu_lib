use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct NEG {
    common: InstructionCommon,
}

impl NEG {
    pub fn new() -> NEG {
        NEG {
            common: InstructionCommon::new(2, 8, true),
        }
    }
}

impl Display for NEG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NEG",)
    }
}

impl BaseInstruction for NEG {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x44]
    }
}

impl ExecutableInstruction<Z80> for NEG {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp.a = alu::neg8(&mut cpu.registers.gp.f, cpu.registers.gp.a);
        cpu.registers.inc_r();
        Ok(())
    }
}
