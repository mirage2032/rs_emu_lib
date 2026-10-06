use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADC_A_PHL {
    common: InstructionCommon,
}

impl ADC_A_PHL {
    pub fn new() -> ADC_A_PHL {
        ADC_A_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for ADC_A_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC A, (HL)",)
    }
}

impl BaseInstruction for ADC_A_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x8E]
    }
}

impl ExecutableInstruction<Z80> for ADC_A_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.gp.hl)?;
        let carry = cpu.registers.gp.f.carry();
        cpu.registers.gp.a = alu::add8(&mut cpu.registers.gp.f, cpu.registers.gp.a, val, carry);
        Ok(())
    }
}
