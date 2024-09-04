use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::push_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct CALL_NN {
    common: InstructionCommon,
    nn: u16,
}

impl CALL_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<CALL_NN, String> {
        Ok(CALL_NN {
            common: InstructionCommon::new(3, 17, false),
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> CALL_NN {
        CALL_NN {
            common: InstructionCommon::new(3, 17, false),
            nn,
        }
    }
}

impl Display for CALL_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CALL 0x{:04x}", self.nn)
    }
}

impl BaseInstruction for CALL_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0xcd, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for CALL_NN {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(cpu.registers.pc.wrapping_add(3), memory, cpu.registers.sp);
        cpu.registers.pc = self.nn;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("cd");
    test_instruction_parse!(CALL_NN, [0xbeef]);
}
