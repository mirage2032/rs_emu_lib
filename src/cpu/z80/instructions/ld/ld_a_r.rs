use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_A_R {
    common: InstructionCommon,
}

impl LD_A_R {
    pub fn new() -> LD_A_R {
        LD_A_R {
            common: InstructionCommon::new(2, 9, true),
        }
    }
}

impl Display for LD_A_R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD A, R",)
    }
}

impl BaseInstruction for LD_A_R {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed,0x5f]
    }
}

impl ExecutableInstruction<Z80> for LD_A_R {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        let r = cpu.registers.r;
        cpu.registers.gp.a = r;
        alu::sz53p(&mut cpu.registers.gp.f, r);
        cpu.registers.gp.f.set_parity_overflow(cpu.interrupts.iff2);
        Ok(())
    }
}
