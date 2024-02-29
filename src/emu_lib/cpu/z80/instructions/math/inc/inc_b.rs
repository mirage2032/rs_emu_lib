use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::inc_r;

pub struct INC_B {
    common: InstructionCommon,
}

impl INC_B {
    pub fn new() -> INC_B {
        INC_B {
            common: InstructionCommon {
                length: 1,
                cycles: 6,
                increment_pc: true,
            },
        }
    }
}

impl Display for INC_B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inc b")
    }
}

impl BaseInstruction for INC_B {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x04]
    }
}

impl ExecutableInstruction<Z80> for INC_B {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) -> Result<(), String> {
        inc_r!(&mut cpu.registers.main.b, &mut cpu.registers.main.f);
        Ok(())
    }
}