use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct SCF {
    common: InstructionCommon,
}

impl SCF {
    pub fn new() -> SCF {
        SCF {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for SCF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCF",)
    }
}

impl BaseInstruction for SCF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x37]
    }
}

impl ExecutableInstruction<Z80> for SCF {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp.f.set_carry(true);
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers
            .gp
            .f
            .set_bit3(((cpu.registers.gp.f.into_bits() | cpu.registers.gp.a) >> 3) & 1 == 1);
        cpu.registers
            .gp
            .f
            .set_bit5(((cpu.registers.gp.f.into_bits() | cpu.registers.gp.a) >> 5) & 1 == 1);
        // cpu.registers.gp.f.set_bit3((((cpu.registers.gp.f.into_bits() | cpu.registers.gp.a) & 0x08) >> 3)==1);
        // cpu.registers.gp.f.set_bit5((((cpu.registers.gp.f.into_bits() | cpu.registers.gp.a) & 0x20) >> 5)==1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("37");
    test_instruction_parse!(SCF);
}
