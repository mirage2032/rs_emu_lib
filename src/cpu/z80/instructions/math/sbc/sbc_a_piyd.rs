use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct SBC_A_PIYD {
    common: InstructionCommon,
    d: i8,
}

impl SBC_A_PIYD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SBC_A_PIYD, MemoryReadError> {
        Ok(SBC_A_PIYD {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> SBC_A_PIYD {
        SBC_A_PIYD {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
        }
    }
}

impl Display for SBC_A_PIYD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SBC A, (IY+0x{:02X})", self.d as u8)
    }
}

impl BaseInstruction for SBC_A_PIYD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0x9e, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for SBC_A_PIYD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let offset = cpu.registers.iy.wrapping_add(self.d as u16);
        let value = memory.read_8(offset as u16)?;
        let carry = cpu.registers.gp.f.carry();
        cpu.registers.gp.a = alu::sub8(&mut cpu.registers.gp.f, cpu.registers.gp.a, value, carry);
        Ok(())
    }
}
