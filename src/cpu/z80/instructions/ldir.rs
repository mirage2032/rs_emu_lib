use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LDIR {
    common: InstructionCommon,
}

impl LDIR {
    pub fn new() -> LDIR {
        LDIR {
            common: InstructionCommon::new(2, 16, true),
        }
    }
}

impl Display for LDIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LDIR",)
    }
}

impl BaseInstruction for LDIR {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0xb0]
    }
}

impl ExecutableInstruction<Z80> for LDIR {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let hl_data = memory.read_8(cpu.registers.gp.hl)?;
        memory.write_8(cpu.registers.gp.de, hl_data)?;
        cpu.registers.gp.hl = cpu.registers.gp.hl.wrapping_add(1);
        cpu.registers.gp.de = cpu.registers.gp.de.wrapping_add(1);
        cpu.registers.gp.bc = cpu.registers.gp.bc.wrapping_sub(1);
        if cpu.registers.gp.bc == 0 {
            self.common.increment_pc = true;
        } else {
            self.common.cycles = 21;
            self.common.increment_pc = false;
        }
        cpu.registers.gp.f.set_add_sub(false);
        cpu.registers.gp.f.set_half_carry(false);
        cpu.registers.gp.f.set_parity_overflow(cpu.registers.gp.bc != 0);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "b0");
    test_instruction_parse!(LDIR);
}
