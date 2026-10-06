use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_A_I {
    common: InstructionCommon,
}

impl LD_A_I {
    pub fn new() -> LD_A_I {
        LD_A_I {
            common: InstructionCommon::new(2, 9, true),
        }
    }
}

impl Display for LD_A_I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD A, I",)
    }
}

impl BaseInstruction for LD_A_I {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed,0x57]
    }
}

impl ExecutableInstruction<Z80> for LD_A_I {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        let i = cpu.registers.i;
        cpu.registers.gp.a = i;
        alu::sz53p(&mut cpu.registers.gp.f, i);
        cpu.registers.gp.f.set_parity_overflow(cpu.interrupts.iff2);
        cpu.registers.inc_r();
        Ok(())
    }
}
