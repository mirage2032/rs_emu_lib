use std::fmt;
use std::fmt::Display;
use crate::cpu::z80::instructions::math::add::generics::add_r_r_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADD_A_PHL {
    common: InstructionCommon,
}

impl ADD_A_PHL {
    pub fn new() -> ADD_A_PHL {
        ADD_A_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for ADD_A_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A, (HL)")
    }
}

impl BaseInstruction for ADD_A_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x86]
    }
}

impl ExecutableInstruction<Z80> for ADD_A_PHL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        add_r_r_setf!(
            &mut cpu.registers.gp[0].a,
            memory.read_8(cpu.registers.gp[0].hl)?,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("86");
    test_instruction_parse!(ADD_A_PHL);
}
