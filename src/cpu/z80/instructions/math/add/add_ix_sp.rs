use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ADD_IX_SP {
    common: InstructionCommon,
}

impl ADD_IX_SP {
    pub fn new() -> ADD_IX_SP {
        ADD_IX_SP {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for ADD_IX_SP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD IX, SP")
    }
}

impl BaseInstruction for ADD_IX_SP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x39]
    }
}

impl ExecutableInstruction<Z80> for ADD_IX_SP {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let (a, b) = (cpu.registers.ix, cpu.registers.sp);
        cpu.registers.ix = alu::add16(&mut cpu.registers.gp.f, a, b);
        cpu.registers.inc_r();
        Ok(())
    }
}
