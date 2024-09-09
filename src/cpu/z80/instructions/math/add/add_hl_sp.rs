use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::add::generics::add_rr_rr_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ADD_HL_SP {
    common: InstructionCommon,
}

impl ADD_HL_SP {
    pub fn new() -> ADD_HL_SP {
        ADD_HL_SP {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for ADD_HL_SP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD HL, SP")
    }
}

impl BaseInstruction for ADD_HL_SP {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x39]
    }
}

impl ExecutableInstruction<Z80> for ADD_HL_SP {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        add_rr_rr_setf!(
            &mut cpu.registers.gp.hl,
            cpu.registers.sp,
            cpu.registers.gp.f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("39");
    test_instruction_parse!(ADD_HL_SP);
}
