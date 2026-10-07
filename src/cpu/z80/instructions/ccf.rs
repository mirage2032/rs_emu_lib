use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct CCF {
    common: InstructionCommon,
}

impl CCF {
    pub fn new() -> CCF {
        CCF {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for CCF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CCF",)
    }
}

impl BaseInstruction for CCF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x3F]
    }
}

impl ExecutableInstruction<Z80> for CCF {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let a = cpu.registers.gp.a;
        alu::ccf(&mut cpu.registers.gp.f, a);
        Ok(())
    }
}
