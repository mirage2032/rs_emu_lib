use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::dec::generics::dec_r_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct DEC_PIYD {
    common: InstructionCommon,
    d: i8,
}

impl DEC_PIYD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<DEC_PIYD,MemoryReadError> {
        Ok(DEC_PIYD {
            common: InstructionCommon::new(3, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8
        })
    }

    pub fn new_with_value(d: i8) -> DEC_PIYD{
        DEC_PIYD{
            common: InstructionCommon::new(3, 23, true),
            d
        }
    }
}

impl Display for DEC_PIYD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC (IY+0x{:02X})",self.d)
    }
}

impl BaseInstruction for DEC_PIYD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xfd,0x35,self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for DEC_PIYD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let offset = cpu.registers.iy.wrapping_add(self.d as u16);
        let mut val = memory.read_8(offset)?;
        dec_r_setf!(&mut val,&mut cpu.registers.gp.f);
        memory.write_8(offset,val)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("fd", "35");
    test_instruction_parse!(DEC_PIYD,[0x2e]);
}
