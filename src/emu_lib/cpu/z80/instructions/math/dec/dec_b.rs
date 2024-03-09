use std::fmt;
use std::fmt::Display;

use crate::dec_r;
use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;
use crate::emu_lib::io::IO;

pub struct DEC_B {
    common: InstructionCommon,
}

impl DEC_B {
    pub fn new() -> DEC_B {
        DEC_B {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for DEC_B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dec b")
    }
}

impl BaseInstruction for DEC_B {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x05]
    }
}

impl ExecutableInstruction<Z80> for DEC_B {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80,_:&mut IO) -> Result<(), String> {
        dec_r!(&mut cpu.registers.main.b, &mut cpu.registers.main.f);
        Ok(())
    }
}