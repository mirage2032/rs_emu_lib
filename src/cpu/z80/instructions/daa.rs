use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct DAA {
    common: InstructionCommon,
}

impl DAA {
    pub fn new() -> DAA {
        DAA {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for DAA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DAA",)
    }
}

impl BaseInstruction for DAA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x27]
    }
}

impl ExecutableInstruction<Z80> for DAA {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let initial_a = cpu.registers.gp.a;
        let mut a = initial_a;

        let carry_in = cpu.registers.gp.f.carry();
        let half_carry_in = cpu.registers.gp.f.half_carry();
        let add_sub = cpu.registers.gp.f.add_sub(); // true for subtraction, false for addition

        let mut correction = 0;

        if add_sub {
            // Subtraction mode
            if half_carry_in || (a & 0x0F) > 9 {
                correction |= 0x06;
            }
            if carry_in || a > 0x99 {
                correction |= 0x60;
                cpu.registers.gp.f.set_carry(true);
            }
            a = a.wrapping_sub(correction);
        } else {
            // Addition mode
            if half_carry_in || (a & 0x0F) > 9 {
                correction |= 0x06;
            }
            if carry_in || a > 0x99 {
                correction |= 0x60;
                cpu.registers.gp.f.set_carry(true);
            }
            a = a.wrapping_add(correction);
        }

        // Update the half-carry flag:
        let new_half_carry = if !add_sub {
            (initial_a & 0x0F) + (correction & 0x0F) > 0x0F
        } else {
            (initial_a & 0x0F) < (correction & 0x0F)
        };
        let parity_flag = a.count_ones() % 2 == 0;
        cpu.registers.gp.a = a;

        // Update the flags
        cpu.registers.gp.f.set_half_carry(new_half_carry);
        cpu.registers.gp.f.set_parity_overflow(parity_flag);
        cpu.registers.gp.f.set_zero(a == 0);
        cpu.registers.gp.f.set_sign(a & 0x80 != 0);

        cpu.registers.gp.f.set_bit3((a >> 3) & 1 == 1);
        cpu.registers.gp.f.set_bit5((a >> 5) & 1 == 1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("27");
    test_instruction_parse!(DAA);
}
