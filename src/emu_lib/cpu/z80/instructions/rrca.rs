use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct RRCA {
    common: InstructionCommon,
}

impl RRCA {
    pub fn new() -> RRCA {
        RRCA {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for RRCA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRCA")
    }
}

impl BaseInstruction for RRCA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0f]
    }
}

impl ExecutableInstruction<Z80> for RRCA {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let carry = cpu.registers.gp[0].a << 7;
        cpu.registers.gp[0].f.set_carry(carry != 0);
        let a = (cpu.registers.gp[0].a >> 1) | carry;
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
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("0f.json");
    test_instruction_parse!(RRCA);
}
