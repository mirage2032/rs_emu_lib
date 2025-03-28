use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::bit::rrc::generics::rrc_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RRC_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl RRC_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<RRC_PIXD, MemoryReadError> {
        Ok(RRC_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> RRC_PIXD {
        RRC_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for RRC_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRC (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for RRC_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xcb, self.d as u8, 0x0e]
    }
}

impl ExecutableInstruction<Z80> for RRC_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let addr = cpu.registers.ix.wrapping_add(self.d as u16);
        let mut value = memory.read_8(addr)?;
        rrc_r_setf!(value, cpu.registers.gp.f);
        memory.write_8(addr, value)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd cb __ 0e");
    test_instruction_parse!(RRC_PIXD, [0xbe]);
}
