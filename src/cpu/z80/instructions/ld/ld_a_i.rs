use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
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
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.registers.gp.a = cpu.registers.i;
        cpu.registers.gp.f.set_parity_overflow(io.iff2);
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers.gp.f.set_sign(cpu.registers.i & 0x80 != 0);
        cpu.registers.gp.f.set_zero(cpu.registers.i == 0);
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 57");
    test_instruction_parse!(LD_A_I);
}
