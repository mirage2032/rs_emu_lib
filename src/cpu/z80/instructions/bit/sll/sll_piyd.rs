use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SLL_PIYD {
    common: InstructionCommon,
    d: i8,
}

impl SLL_PIYD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SLL_PIYD, MemoryReadError> {
        Ok(SLL_PIYD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> SLL_PIYD {
        SLL_PIYD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for SLL_PIYD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SLL (IY+0x{:02X})", self.d)
    }
}

impl BaseInstruction for SLL_PIYD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd, 0xcb, self.d as u8, 0x36]
    }
}

impl ExecutableInstruction<Z80> for SLL_PIYD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let addr = cpu.registers.iy.wrapping_add(self.d as u16);
        let mut value = memory.read_8(addr)?;
        value = alu::sll8(&mut cpu.registers.gp.f, value);
        memory.write_8(addr, value)?;
        Ok(())
    }
}
