use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_I_A {
    common: InstructionCommon,
}

impl LD_I_A {
    pub fn new() -> LD_I_A {
        LD_I_A {
            common: InstructionCommon::new(2, 9, true),
        }
    }
}

impl Display for LD_I_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD I, A",)
    }
}

impl BaseInstruction for LD_I_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed,0x47]
    }
}

impl ExecutableInstruction<Z80> for LD_I_A {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.i = cpu.registers.gp.a;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 47");
    test_instruction_parse!(LD_I_A);
}
