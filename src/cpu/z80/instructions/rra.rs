use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RRA {
    common: InstructionCommon,
}

impl RRA {
    pub fn new() -> RRA {
        RRA {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for RRA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRA")
    }
}

impl BaseInstruction for RRA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x1f]
    }
}

impl ExecutableInstruction<Z80> for RRA {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let carry = cpu.registers.gp.a & 1;
        let a = (cpu.registers.gp.a >> 1) | (cpu.registers.gp.f.carry() as u8) << 7;
        cpu.registers.gp.f.set_carry(carry != 0);
        cpu.registers.gp.a = a;
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers.gp.f.set_bit3((a >> 3) & 1 == 1);
        cpu.registers.gp.f.set_bit5((a >> 5) & 1 == 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("1f");
    test_instruction_parse!(RRA);
}
