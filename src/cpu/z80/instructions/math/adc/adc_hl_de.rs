use crate::cpu::z80::instructions::math::adc::hex;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::instructions::math::adc::generics::adc_rr_rr_setf;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADC_HL_DE {
    common: InstructionCommon,
}

impl ADC_HL_DE {
    pub fn new() -> ADC_HL_DE {
        ADC_HL_DE {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for ADC_HL_DE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC HL, DE")
    }
}

impl BaseInstruction for ADC_HL_DE {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x5A]
    }
}

impl ExecutableInstruction<Z80> for ADC_HL_DE {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        adc_rr_rr_setf!(
            &mut cpu.registers.gp.hl,
            cpu.registers.gp.de,
            &mut cpu.registers.gp.f
        );
        cpu.registers.r = cpu.registers.r.wrapping_add(1) % 0x80;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 5a");
    test_instruction_parse!(ADC_HL_DE);
}
