use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::add::generics::add_r_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADD_A_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl ADD_A_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<ADD_A_PIXD, MemoryReadError> {
        Ok(ADD_A_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> ADD_A_PIXD {
        ADD_A_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
        }
    }
}

impl Display for ADD_A_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A, (IX+0x{:02X})", self.d as u8)
    }
}

impl BaseInstruction for ADD_A_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x86, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for ADD_A_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let offset = cpu.registers.ix.wrapping_add(self.d as u16);
        let value = memory.read_8(offset as u16)?;
        add_r_r_setf!(&mut cpu.registers.gp.a, value, &mut cpu.registers.gp.f);
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd 86");
    test_instruction_parse!(ADD_A_PIXD, [0x53]);
}
