use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_PNN_BC {
    common: InstructionCommon,
    nn: u16,
}

impl LD_PNN_BC {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_PNN_BC, MemoryReadError> {
        Ok(LD_PNN_BC {
            common: InstructionCommon::new(4, 20, true),
            nn: memory.read_16(pos.wrapping_add(2))?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_PNN_BC {
        LD_PNN_BC {
            common: InstructionCommon::new(4, 20, true),
            nn,
        }
    }
}

impl Display for LD_PNN_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (0x{:04X}), BC", self.nn)
    }
}

impl BaseInstruction for LD_PNN_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0xED, 0x43, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_PNN_BC {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        memory.write_16(self.nn, cpu.registers.gp.bc)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 43");
    test_instruction_parse!(LD_PNN_BC, [0xbeef]);
}
