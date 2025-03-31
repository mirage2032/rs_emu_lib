use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct DJNZ_D {
    common: InstructionCommon,
    d: i8,
}

impl DJNZ_D {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<DJNZ_D, MemoryReadError> {
        Ok(DJNZ_D {
            common: InstructionCommon::new(2, 8, true),
            d: memory.read_8(pos.wrapping_add(1))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> DJNZ_D {
        DJNZ_D {
            common: InstructionCommon::new(2, 8, true),
            d: d as i8,
        }
    }
}

impl Display for DJNZ_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DJNZ 0x{:02X}", self.d)
    }
}

impl BaseInstruction for DJNZ_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x10, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for DJNZ_D {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp.b = cpu.registers.gp.b.wrapping_sub(1);
        if cpu.registers.gp.b != 0 {
            self.common = InstructionCommon::new(2, 13, true);
            cpu.registers.pc = cpu.registers.pc.wrapping_add(self.d as u16);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("10");
    test_instruction_parse!(DJNZ_D, [0xbf]);
}
