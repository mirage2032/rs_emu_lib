use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;

pub struct LD_BC_NN {
    common: InstructionCommon,
    nn: u16,
}

impl LD_BC_NN {
    pub fn new(memory: &Memory, pos: u16) -> LD_BC_NN {
        LD_BC_NN {
            common: InstructionCommon {
                length: 3,
                cycles: 10,
                increment_pc: true,
            },
            nn: memory.read16(pos + 1),
        }
    }
}

impl Display for LD_BC_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ld bc, {:x}", self.nn)
    }
}

impl BaseInstruction for LD_BC_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
}

impl ExecutableInstruction<Z80> for LD_BC_NN {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80) {
        cpu.registers.main.bc.set(self.nn);
    }
}