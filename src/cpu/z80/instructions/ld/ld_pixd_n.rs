use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::registers::BaseRegister;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_PIXD_N {
    common: InstructionCommon,
    n: u8,
    d: i8,
}

impl LD_PIXD_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_PIXD_N, String> {
        Ok(LD_PIXD_N {
            common: InstructionCommon::new(4, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
            n: memory.read_8(pos.wrapping_add(3))?,
        })
    }

    pub fn new_with_value(d: u8, n: u8) -> LD_PIXD_N {
        LD_PIXD_N {
            common: InstructionCommon::new(4, 19, true),
            n,
            d: d as i8,
        }
    }
}

impl Display for LD_PIXD_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (IX+0x{:02X}), 0x{:02X}", self.d, self.n)
    }
}

impl BaseInstruction for LD_PIXD_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xDD, 0x36, self.d as u8, self.n]
    }
}

impl ExecutableInstruction<Z80> for LD_PIXD_N {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get_mut("ix") {
            Some(BaseRegister::Bit16(val)) => {
                memory.write_8(val.wrapping_add(self.d as u16), self.n)?;
            }
            _ => return Err("Invalid register".to_string()),
        }
        match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => {
                *val = val.wrapping_add(1) % 128;
            }
            _ => return Err("Invalid register".to_string()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd 36");
    test_instruction_parse!(LD_PIXD_N, [0xbe, 0xef]);
}
