use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::push_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct CALL_PO_NN {
    common: InstructionCommon,
    nn: u16,
}

impl CALL_PO_NN {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<CALL_PO_NN, MemoryReadError> {
        Ok(CALL_PO_NN {
            common: InstructionCommon::new(3, 10, true),
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> CALL_PO_NN {
        CALL_PO_NN {
            common: InstructionCommon::new(3, 10, true),
            nn,
        }
    }
}

impl Display for CALL_PO_NN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CALL PO, 0x{:04X}", self.nn)
    }
}

impl BaseInstruction for CALL_PO_NN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0xe4, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for CALL_PO_NN {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        if !cpu.registers.gp.f.parity_overflow() {
            self.common = InstructionCommon::new(3, 17, false);
            push_16!(cpu.registers.pc.wrapping_add(3), memory, cpu.registers.sp);
            cpu.registers.pc = self.nn;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("e4");
    test_instruction_parse!(CALL_PO_NN, [0xbeef]);
}
