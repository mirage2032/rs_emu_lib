use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct CPL {
    common: InstructionCommon,
}

impl CPL {
    pub fn new() -> CPL {
        CPL {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for CPL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CPL")
    }
}

impl BaseInstruction for CPL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x2f]
    }
}

impl ExecutableInstruction<Z80> for CPL {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let inverted_a = !cpu.registers.gp.a;
        cpu.registers.gp.a = inverted_a;

        cpu.registers.gp.f.set_half_carry(true);
        cpu.registers.gp.f.set_add_sub(true);

        cpu.registers.gp.f.set_bit3((inverted_a >> 3) & 1 == 1);
        cpu.registers.gp.f.set_bit5((inverted_a >> 5) & 1 == 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("1f");
    test_instruction_parse!(CPL);
}
