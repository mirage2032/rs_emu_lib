use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::errors::MemoryReadError;

#[derive(Debug)]
pub struct LD_SP_PNN {
    common: InstructionCommon,
    nn: u16,
}

impl LD_SP_PNN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_SP_PNN, MemoryReadError> {
        Ok(LD_SP_PNN {
            common: InstructionCommon::new(4, 20, true),
            nn: memory.read_16(pos.wrapping_add(2))?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_SP_PNN {
        LD_SP_PNN {
            common: InstructionCommon::new(4, 20, true),
            nn,
        }
    }
}

impl Display for LD_SP_PNN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, (0x{:04x})", self.nn)
    }
}

impl BaseInstruction for LD_SP_PNN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0xED, 0x7b, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_SP_PNN {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = memory.read_16(self.nn)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 7b");
    test_instruction_parse!(LD_SP_PNN, [0xbeef]);
}
