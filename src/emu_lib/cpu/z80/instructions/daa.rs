use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

#[derive(Debug)]
pub struct DAA {
    common: InstructionCommon,
}

impl DAA {
    pub fn new() -> DAA {
        DAA { common: *COMMON }
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
        let initial_a = cpu.registers.gp[0].a;
        let mut a = initial_a;

        let carry_in = cpu.registers.gp[0].f.carry();
        let half_carry_in = cpu.registers.gp[0].f.half_carry();
        let add_sub = cpu.registers.gp[0].f.add_sub(); // true for subtraction, false for addition

        let mut correction = 0;

        if add_sub {
            // Subtraction mode
            if half_carry_in || (a & 0x0F) > 9 {
                correction |= 0x06;
            }
            if carry_in || a > 0x99 {
                correction |= 0x60;
                cpu.registers.gp[0].f.set_carry(true);
            }
            a = a.wrapping_sub(correction);
        } else {
            // Addition mode
            if half_carry_in || (a & 0x0F) > 9 {
                correction |= 0x06;
            }
            if carry_in || a > 0x99 {
                correction |= 0x60;
                cpu.registers.gp[0].f.set_carry(true);
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
        cpu.registers.gp[0].a = a;

        // Update the flags
        cpu.registers.gp[0].f.set_half_carry(new_half_carry);
        cpu.registers.gp[0].f.set_parity_overflow(parity_flag);
        cpu.registers.gp[0].f.set_zero(a == 0);
        cpu.registers.gp[0].f.set_sign(a & 0x80 != 0);
        
        cpu.registers.gp[0].f.set_bit3((a >> 3) & 1 == 1);
        cpu.registers.gp[0].f.set_bit5((a >> 5) & 1 == 1);

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("27");
    test_instruction_parse!(DAA);
}
