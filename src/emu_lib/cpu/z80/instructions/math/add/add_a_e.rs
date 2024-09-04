use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADD_A_E {
    common: InstructionCommon,
}

impl ADD_A_E {
    pub fn new() -> ADD_A_E {
        ADD_A_E {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for ADD_A_E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A, E")
    }
}

impl BaseInstruction for ADD_A_E {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x83]
    }
}

impl ExecutableInstruction<Z80> for ADD_A_E {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::add_r_r!(
            &mut cpu.registers.gp[0].a,
            cpu.registers.gp[0].e,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("83");
    test_instruction_parse!(ADD_A_E);
}
