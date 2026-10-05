use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
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
        cpu.registers.inc_r(); // the second opcode fetch counts before R is read
        let r = cpu.registers.r;
        cpu.registers.gp.a = r;
        cpu.registers.gp.f.set_parity_overflow(cpu.interrupts.iff2);
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers.gp.f.set_sign(r & 0x80 != 0);
        cpu.registers.gp.f.set_zero(r == 0);
        cpu.registers.gp.f.set_add_sub(false);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 5f");
    test_instruction_parse!(LD_A_R);
}
