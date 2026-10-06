use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct INC_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl INC_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<INC_PIXD, MemoryReadError> {
        Ok(INC_PIXD {
            common: InstructionCommon::new(3, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: i8) -> INC_PIXD {
        INC_PIXD {
            common: InstructionCommon::new(3, 23, true),
            d,
        }
    }
}

impl Display for INC_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for INC_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x34, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for INC_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let address = cpu.registers.ix.wrapping_add(self.d as u16);
        let value = memory.read_8(address)?;
        let result = alu::inc8(&mut cpu.registers.gp.f, value);
        memory.write_8(address, result)?;
        Ok(())
    }
}
