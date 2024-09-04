use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(2, 7, true));

#[derive(Debug)]
pub struct JR_Z_D {
    common: InstructionCommon,
    d: i8,
}

impl JR_Z_D {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<JR_Z_D, String> {
        Ok(JR_Z_D {
            common: *COMMON,
            d: memory.read_8(pos.wrapping_add(1))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> JR_Z_D {
        JR_Z_D {
            common: *COMMON,
            d: d as i8,
        }
    }
}

impl Display for JR_Z_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JR Z, 0x{:02x}", self.d)
    }
}

impl BaseInstruction for JR_Z_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x28, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for JR_Z_D {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if cpu.registers.gp[0].f.zero() {
            self.common = InstructionCommon::new(2, 12, true);
            cpu.registers.pc = cpu.registers.pc.wrapping_add(self.d as u16);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;
    test_z80!("28");
    test_instruction_parse!(JR_Z_D, [0xbf]);
}
