use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::registers::BaseRegister;
use crate::cpu::z80::instructions::math::add::generics::add_r_r_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADD_A_PIXD {
    common: InstructionCommon,
    d: i8,
}

impl ADD_A_PIXD {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<ADD_A_PIXD, String> {
        Ok(ADD_A_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> ADD_A_PIXD {
        ADD_A_PIXD {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
        }
    }
}

impl Display for ADD_A_PIXD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A, (IX+0x{:02x})", self.d as u8)
    }
}

impl BaseInstruction for ADD_A_PIXD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x86, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for ADD_A_PIXD {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get_mut("ix") {
            Some(crate::cpu::registers::BaseRegister::Bit16(val)) => {
                let offset = val.wrapping_add(self.d as u16);
                let value = memory.read_8(offset as u16)?;
                add_r_r_setf!(
                    &mut cpu.registers.gp[0].a,
                    value,
                    &mut cpu.registers.gp[0].f
                );
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
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("dd 86");
    test_instruction_parse!(ADD_A_PIXD, [0x53]);
}
