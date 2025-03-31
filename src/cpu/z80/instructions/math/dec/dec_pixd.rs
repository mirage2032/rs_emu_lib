use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct DEC_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl DEC_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<DEC_PIXD, MemoryReadError> {
        Ok(DEC_PIXD {
            common: InstructionCommon::new(3, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: i8) -> DEC_PIXD {
        DEC_PIXD {
            common: InstructionCommon::new(2, 23, true),
            d,
        }
    }
}

impl Display for DEC_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for DEC_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x35, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for DEC_PIXD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let mem_addr = cpu.registers.ix.wrapping_add(self.d as u16);
        let val = memory.read_8(mem_addr)?;
        cpu.registers.gp.f.set_half_carry(val & 0x0f == 0);
        let val = val.wrapping_sub(1);
        cpu.registers.gp.f.set_sign(val & 0x80 != 0);
        cpu.registers.gp.f.set_parity_overflow(val == 0x7F);
        cpu.registers.gp.f.set_zero(val == 0);
        cpu.registers.gp.f.set_add_sub(true);
        memory.write_8(mem_addr, val)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd", "35");
    test_instruction_parse!(DEC_PIXD, [0x10]);
}
