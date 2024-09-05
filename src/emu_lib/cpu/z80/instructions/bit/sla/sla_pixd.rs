use std::fmt;
use std::fmt::Display;

use crate::cpu::z80::instructions::bit::sla::generics::sla_r_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};
use crate::cpu::z80::BaseRegister;

#[derive(Debug)]
pub struct SLA_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl SLA_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<SLA_PIXD,String> {
        Ok(SLA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> SLA_PIXD {
        SLA_PIXD {
            common: InstructionCommon::new(4, 23, true),
            d: d as i8,
        }
    }
}

impl Display for SLA_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SLA (IX+0x{:02X})",self.d)
    }
}

impl BaseInstruction for SLA_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xcb,self.d as u8, 0x2e]
    }
}

impl ExecutableInstruction<Z80> for SLA_PIXD {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(ix)) => {
                let addr = ix.wrapping_add(self.d as u16);
                let mut value = memory.read_8(addr)?;
                sla_r_setf!(value, cpu.registers.gp[0].f);
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
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("dd cb __ 26");
    test_instruction_parse!(SLA_PIXD, [0xbe]);
}
