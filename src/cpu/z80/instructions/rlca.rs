use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RLCA {
    common: InstructionCommon,
}

impl RLCA {
    pub fn new() -> RLCA {
        RLCA {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for RLCA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLCA")
    }
}

impl BaseInstruction for RLCA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x07]
    }
}

impl ExecutableInstruction<Z80> for RLCA {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let carry = cpu.registers.gp[0].a >> 7;
        cpu.registers.gp[0].f.set_carry(carry != 0);
        let a = (cpu.registers.gp[0].a << 1) | carry;
        cpu.registers.gp[0].a = a;
        cpu.registers.gp[0].f.set_add_sub(false);
        cpu.registers.gp[0].f.set_half_carry(false);
        cpu.registers.gp[0].f.set_bit3((a >> 3) & 1 == 1);
        cpu.registers.gp[0].f.set_bit5((a >> 5) & 1 == 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::test_instruction_parse;
    use crate::cpu::z80::test::*;

    test_z80!("07");
    test_instruction_parse!(RLCA);
}
