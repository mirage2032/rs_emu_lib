use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct JP_PO_NN {
    common: InstructionCommon,
    nn: u16,
}

impl JP_PO_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<JP_PO_NN, MemoryReadError> {
        Ok(JP_PO_NN {
            common: InstructionCommon::new(3, 10, true),
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> JP_PO_NN {
        JP_PO_NN {
            common: InstructionCommon::new(3, 10, true),
            nn,
        }
    }
}

impl Display for JP_PO_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP PO, 0x{:04x}", self.nn)
    }
}

impl BaseInstruction for JP_PO_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = self.nn.to_le_bytes();
        vec![0xE2, bytes[0], bytes[1]]
    }
}

impl ExecutableInstruction<Z80> for JP_PO_NN {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if !cpu.registers.gp.f.parity_overflow() {
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

    test_z80!("e2");
    test_instruction_parse!(JP_PO_NN, [0xbeef]);
}
