use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADC_A_B {
    common: InstructionCommon,
}

impl ADC_A_B {
    pub fn new() -> ADC_A_B {
        ADC_A_B {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for ADC_A_B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC A, B")
    }
}

impl BaseInstruction for ADC_A_B {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x88]
    }
}

impl ExecutableInstruction<Z80> for ADC_A_B {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::adc_r_r!(
            &mut cpu.registers.gp[0].a,
            cpu.registers.gp[0].b,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("88.json");
    test_instruction_parse!(ADC_A_B);
}
