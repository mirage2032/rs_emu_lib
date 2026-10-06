use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct SCF {
    common: InstructionCommon,
}

impl SCF {
    pub fn new() -> SCF {
        SCF {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for SCF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCF",)
    }
}

impl BaseInstruction for SCF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x37]
    }
}

impl ExecutableInstruction<Z80> for SCF {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let a = cpu.registers.gp.a;
        alu::scf(&mut cpu.registers.gp.f, a);
        Ok(())
    }
}
