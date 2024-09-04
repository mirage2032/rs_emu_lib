use std::fmt;
use std::fmt::Display;

use crate::cpu::registers::BaseRegister;
use crate::cpu::z80::instructions::math::add::generics::add_rr_rr_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADD_IX_SP {
    common: InstructionCommon,
}

impl ADD_IX_SP {
    pub fn new() -> ADD_IX_SP {
        ADD_IX_SP {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for ADD_IX_SP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD IX, SP")
    }
}

impl BaseInstruction for ADD_IX_SP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0x39]
    }
}

impl ExecutableInstruction<Z80> for ADD_IX_SP {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get_mut("ix") {
            Some(crate::cpu::registers::BaseRegister::Bit16(val)) => {
                add_rr_rr_setf!(val, cpu.registers.sp, cpu.registers.gp[0].f);
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

    test_z80!("dd 39");
    test_instruction_parse!(ADD_IX_SP);
}
