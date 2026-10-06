use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SLA_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl SLA_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SLA_PIXD, MemoryReadError> {
        Ok(SLA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> SLA_PIXD {
        SLA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for SLA_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SLA (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for SLA_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xcb, self.d as u8, 0x26]
    }
}

impl ExecutableInstruction<Z80> for SLA_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let addr = cpu.registers.ix.wrapping_add(self.d as u16);
        let mut value = memory.read_8(addr)?;
        value = alu::sla8(&mut cpu.registers.gp.f, value);
        memory.write_8(addr, value)?;
        cpu.registers.inc_r();
        Ok(())
    }
}
