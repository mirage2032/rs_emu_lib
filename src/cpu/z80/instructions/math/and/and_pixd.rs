use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::and::and_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct AND_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl AND_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<AND_PIXD, MemoryReadError> {
        Ok(AND_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> AND_PIXD {
        AND_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
        }
    }
}

impl Display for AND_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AND (IX+0x{:02x})", self.d)
    }
}

impl BaseInstruction for AND_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xa6, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for AND_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.ix.wrapping_add(self.d as u16))?;
        and_r_setf!(cpu.registers.gp.a, val, cpu.registers.gp.f);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd", "a6");
    test_instruction_parse!(AND_PIXD, [0xbf]);
}
