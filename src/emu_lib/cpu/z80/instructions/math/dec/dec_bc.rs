use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::emu_lib::io::IO;

pub struct DEC_BC {
    common: InstructionCommon,
}

impl DEC_BC {
    pub fn new() -> DEC_BC {
        DEC_BC {
            common: InstructionCommon {
                length: 1,
                cycles: 6,
                increment_pc: true,
            },
        }
    }
}

impl Display for DEC_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dec bc")
    }
}

impl BaseInstruction for DEC_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0b]
    }
}

impl ExecutableInstruction<Z80> for DEC_BC {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.main.bc = cpu.registers.main.bc.wrapping_sub(1);
        Ok(())
    }
}