use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;
use crate::cpu::registers::BaseRegister;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(3, 19, true));

#[derive(Debug)]
pub struct LD_IXPD_A {
    common: InstructionCommon,
    d: i8,
}

impl LD_IXPD_A {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_IXPD_A, String> {
        Ok(LD_IXPD_A {
            common: *COMMON,
            d: memory.read_8(pos + 2)? as i8,
        })
    }

    pub fn new_with_value(d: u8) -> LD_IXPD_A {
        LD_IXPD_A {
            common: *COMMON,
            d: d as i8,
        }
    }
}

impl Display for LD_IXPD_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (IX+0x{:02x}), A", self.d)
    }
}

impl BaseInstruction for LD_IXPD_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xDD,0x77, self.d as u8]
    }
}

impl ExecutableInstruction<Z80> for LD_IXPD_A {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(ix)) => {
                let addr = ix.wrapping_add(self.d as u16);
                memory.write_8(addr, cpu.registers.gp[0].a)?;
            }
            _ => {
                return Err("IX register not found".to_string());
            }
        }
        match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => { *val = val.wrapping_add(1) % 128; },
            _ => return Err("Invalid register".to_string()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("dd 77.json");
    test_instruction_parse!(LD_IXPD_A, [0x44]);
}
