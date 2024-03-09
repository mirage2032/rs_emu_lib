use std::fmt;
use std::fmt::Display;

use crate::dec_r;
use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::emu_lib::io::IO;

pub struct DEC_C {
    common: InstructionCommon,
}

impl DEC_C {
    pub fn new() -> DEC_C {
        DEC_C {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for DEC_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dec c")
    }
}

impl BaseInstruction for DEC_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0d]
    }
}

impl ExecutableInstruction<Z80> for DEC_C {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80,_:&mut  IO) -> Result<(), String> {
        dec_r!(&mut cpu.registers.main.c, &mut cpu.registers.main.f);
        Ok(())
    }
}