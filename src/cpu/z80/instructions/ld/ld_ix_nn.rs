use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_IX_NN {
    common: InstructionCommon,
    nn: u16,
}

impl LD_IX_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_IX_NN, String> {
        Ok(LD_IX_NN {
            common: InstructionCommon::new(4, 14, true),
            nn: memory.read_16(pos.wrapping_add(2))?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_IX_NN {
        LD_IX_NN {
            common: InstructionCommon::new(4, 14, true),
            nn,
        }
    }
}

impl Display for LD_IX_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD IX, 0x{:04x}", self.nn)
    }
}

impl BaseInstruction for LD_IX_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0xDD, 0x21, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_IX_NN {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                cpu.registers.ix = self.nn;
                cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd 21");
    test_instruction_parse!(LD_IX_NN, [0xbeef]);
}
