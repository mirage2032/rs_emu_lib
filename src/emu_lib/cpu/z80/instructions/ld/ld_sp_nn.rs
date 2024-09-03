use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(3, 10, true));

#[derive(Debug)]
pub struct LD_SP_NN {
    common: InstructionCommon,
    nn: u16,
}

impl LD_SP_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_SP_NN, String> {
        Ok(LD_SP_NN {
            common: *COMMON,
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_SP_NN {
        LD_SP_NN {
            common: *COMMON,
            nn,
        }
    }
}

impl Display for LD_SP_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, 0x{:04x}", self.nn)
    }
}

impl BaseInstruction for LD_SP_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0x31, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_SP_NN {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = self.nn;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("31.json");
    test_instruction_parse!(LD_SP_NN, [0xbeef]);
}
