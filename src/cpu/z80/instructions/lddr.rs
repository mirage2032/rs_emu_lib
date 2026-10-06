use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LDDR {
    common: InstructionCommon,
}

impl LDDR {
    pub fn new() -> LDDR {
        LDDR {
            common: InstructionCommon::new(2, 16, true),
        }
    }
}

impl Display for LDDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LDDR",)
    }
}

impl BaseInstruction for LDDR {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0xb8]
    }
}

impl ExecutableInstruction<Z80> for LDDR {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let value = memory.read_8(cpu.registers.gp.hl)?;
        memory.write_8(cpu.registers.gp.de, value)?;
        cpu.registers.gp.hl = cpu.registers.gp.hl.wrapping_sub(1);
        cpu.registers.gp.de = cpu.registers.gp.de.wrapping_sub(1);
        cpu.registers.gp.bc = cpu.registers.gp.bc.wrapping_sub(1);
        if cpu.registers.gp.bc == 0 {
            self.common.increment_pc = true;
        } else {
            self.common.cycles = 21;
            self.common.increment_pc = false;
        }
        let (a, bc) = (cpu.registers.gp.a, cpu.registers.gp.bc);
        alu::block_ld(&mut cpu.registers.gp.f, a, value, bc);
        if bc != 0 {
            alu::block_repeat(&mut cpu.registers.gp.f, cpu.registers.pc);
        }
        Ok(())
    }
}
