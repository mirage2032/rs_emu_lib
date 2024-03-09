use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::{Memory, ReadableMemory};
use crate::emu_lib::io::IO;

const COMMON: InstructionCommon = InstructionCommon {
    length: 3,
    cycles: 10,
    increment_pc: true,
};

pub struct LD_BC_NN {
    common: InstructionCommon,
    nn: u16,
}

impl LD_BC_NN {
    pub fn new<T: ReadableMemory>(memory: &T, pos: u16) -> Result<LD_BC_NN, String>
    {
        Ok(LD_BC_NN {
            common: COMMON,
            nn: memory.read_16(pos + 1)?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_BC_NN {
        LD_BC_NN {
            common: COMMON,
            nn,
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
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0x01, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_BC_NN {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _:&mut  IO) -> Result<(), String> {
        cpu.registers.main.bc = self.nn;
        Ok(())
    }
}