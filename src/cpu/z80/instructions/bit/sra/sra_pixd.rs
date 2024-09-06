use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::bit::sra::generics::sra_r_setf;
use crate::cpu::z80::BaseRegister;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SRA_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl SRA_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SRA_PIXD, String> {
        Ok(SRA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> SRA_PIXD {
        SRA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for SRA_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRA (IX+0x{:02X})", self.d)
    }
}

impl BaseInstruction for SRA_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xcb, self.d as u8, 0x2e]
    }
}

impl ExecutableInstruction<Z80> for SRA_PIXD {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(ix)) => {
                let addr = ix.wrapping_add(self.d as u16);
                let mut value = memory.read_8(addr)?;
                sra_r_setf!(value, cpu.registers.gp[0].f);
                memory.write_8(addr, value)?;
            }
            _ => {
                return Err("IX register not found".to_string());
            }
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

    test_z80!("dd cb __ 2e");
    test_instruction_parse!(SRA_PIXD, [0xbe]);
}