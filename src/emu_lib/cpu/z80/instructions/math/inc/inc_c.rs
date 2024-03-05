use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::inc_r;
use crate::io::IO;

pub struct INC_C {
    common: InstructionCommon,
}

impl INC_C {
    pub fn new() -> INC_C {
        INC_C {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for INC_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inc c")
    }
}

impl BaseInstruction for INC_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0c]
    }
}

impl ExecutableInstruction<Z80> for INC_C {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80,_:&mut IO) -> Result<(), String> {
        inc_r!(&mut cpu.registers.main.c, &mut cpu.registers.main.f);
        Ok(())
    }
}