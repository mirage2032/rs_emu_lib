use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IN_L_C {
    common: InstructionCommon,
}

impl IN_L_C {
    pub fn new() -> IN_L_C {
        IN_L_C {
            common: InstructionCommon::new(2, 12, true),
        }
    }
}

impl Display for IN_L_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IN L, (C)")
    }
}

impl BaseInstruction for IN_L_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x68]
    }
}

impl ExecutableInstruction<Z80> for IN_L_C {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.registers.gp.l = io.read(cpu.registers.gp.c)?;
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers
            .gp
            .f
            .set_parity_overflow(cpu.registers.gp.l.count_ones() % 2 == 0);
        cpu.registers.gp.f.set_zero(cpu.registers.gp.l == 0);
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers.gp.f.set_sign(cpu.registers.gp.l & 0x80 != 0);
        cpu.registers.r = cpu.registers.r.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 68");
    test_instruction_parse!(IN_L_C);
}
