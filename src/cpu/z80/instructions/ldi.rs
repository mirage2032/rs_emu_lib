use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LDI {
    common: InstructionCommon,
}

impl LDI {
    pub fn new() -> LDI {
        LDI {
            common: InstructionCommon::new(2, 16, true),
        }
    }
}

impl Display for LDI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LDI",)
    }
}

impl BaseInstruction for LDI {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0xa0]
    }
}

impl ExecutableInstruction<Z80> for LDI {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let hl_data = memory.read_8(cpu.registers.gp.hl)?;
        memory.write_8(cpu.registers.gp.de, hl_data)?;
        cpu.registers.gp.hl = cpu.registers.gp.hl.wrapping_add(1);
        cpu.registers.gp.de = cpu.registers.gp.de.wrapping_add(1);
        cpu.registers.gp.bc = cpu.registers.gp.bc.wrapping_sub(1);
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers.gp.f.set_half_carry(false);
        if cpu.registers.gp.bc == 0 {
            cpu.registers.gp.f.set_parity_overflow(false);
        } else {
            cpu.registers.gp.f.set_parity_overflow(true);
        }
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "a0");
    test_instruction_parse!(LDI);
}
