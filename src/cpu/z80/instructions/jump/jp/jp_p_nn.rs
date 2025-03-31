use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct JP_P_NN {
    common: InstructionCommon,
    nn: u16,
}

impl JP_P_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<JP_P_NN, MemoryReadError> {
        Ok(JP_P_NN {
            common: InstructionCommon::new(3, 10, true),
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> JP_P_NN {
        JP_P_NN {
            common: InstructionCommon::new(3, 10, true),
            nn,
        }
    }
}

impl Display for JP_P_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP P, 0x{:04X}", self.nn)
    }
}

impl BaseInstruction for JP_P_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = self.nn.to_le_bytes();
        vec![0xF2, bytes[0], bytes[1]]
    }
}

impl ExecutableInstruction<Z80> for JP_P_NN {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if !cpu.registers.gp.f.sign() {
            self.common.increment_pc = false;
            cpu.registers.pc = self.nn;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("f2");
    test_instruction_parse!(JP_P_NN, [0xbeef]);
}
