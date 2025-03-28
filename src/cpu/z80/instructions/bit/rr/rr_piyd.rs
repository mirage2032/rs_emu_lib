use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::bit::rr::generics::rr_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct RR_PIYD {
    common: InstructionCommon,
    d: i8,
}

impl RR_PIYD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<RR_PIYD, MemoryReadError> {
        Ok(RR_PIYD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> RR_PIYD {
        RR_PIYD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for RR_PIYD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RR (IY+0x{:02X})", self.d)
    }
}

impl BaseInstruction for RR_PIYD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0xcb, self.d as u8, 0x1e]
    }
}

impl ExecutableInstruction<Z80> for RR_PIYD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let addr = cpu.registers.iy.wrapping_add(self.d as u16);
        let mut value = memory.read_8(addr)?;
        rr_r_setf!(value, cpu.registers.gp.f);
        memory.write_8(addr, value)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd cb __ 1e");
    test_instruction_parse!(RR_PIYD, [0xbe]);
}
