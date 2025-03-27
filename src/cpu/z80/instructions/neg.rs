use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
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
        vec![0xed,0x44]
    }
}

impl ExecutableInstruction<Z80> for NEG {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp.a = cpu.registers.gp.a.wrapping_neg();
        cpu.registers.gp.f.set_parity_overflow(cpu.registers.gp.a == 0x80);
        cpu.registers.gp.f.set_zero(cpu.registers.gp.a == 0);
        cpu.registers.gp.f.set_carry(cpu.registers.gp.a != 0);
        cpu.registers.gp.f.set_half_carry(cpu.registers.gp.a & 0x0f != 0);
        cpu.registers.gp.f.set_add_sub(true);
        cpu.registers.gp.f.set_sign(cpu.registers.gp.a & 0x80 != 0);
        cpu.registers.r = cpu.registers.r.wrapping_add(1)%128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed","44");
    test_instruction_parse!(NEG);
}
