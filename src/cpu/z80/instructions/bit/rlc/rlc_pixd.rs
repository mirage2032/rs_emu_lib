use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RLC_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl RLC_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<RLC_PIXD, MemoryReadError> {
        Ok(RLC_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> RLC_PIXD {
        RLC_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for RLC_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLC (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for RLC_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xcb, self.d as u8, 0x06]
    }
}

impl ExecutableInstruction<Z80> for RLC_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let addr = cpu.registers.ix.wrapping_add(self.d as u16);
        let mut value = memory.read_8(addr)?;
        value = alu::rlc8(&mut cpu.registers.gp.f, value);
        memory.write_8(addr, value)?;
        cpu.registers.inc_r();
        Ok(())
    }
}
