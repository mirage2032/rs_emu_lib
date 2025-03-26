use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;
use crate::cpu::z80::instructions::math::cp::cp_r_setf;

#[derive(Debug)]
pub struct CP_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl CP_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<CP_PIXD, MemoryReadError> {
        Ok(CP_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> CP_PIXD {
        CP_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
        }
    }
}

impl Display for CP_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CP (IX+0x{:02x})", self.d as u8)
    }
}

impl BaseInstruction for CP_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xbe, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for CP_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let offset = cpu.registers.ix.wrapping_add(self.d as u16);
        let value = memory.read_8(offset as u16)?;
        cp_r_setf!(cpu.registers.gp.a, value, cpu.registers.gp.f);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd be");
    test_instruction_parse!(CP_PIXD, [0x53]);
}
