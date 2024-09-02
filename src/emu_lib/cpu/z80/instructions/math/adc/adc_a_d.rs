use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct ADC_A_D {
    common: InstructionCommon,
}

impl ADC_A_D {
    pub fn new() -> ADC_A_D {
        ADC_A_D {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for ADC_A_D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC A, D")
    }
}

impl BaseInstruction for ADC_A_D {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x8a]
    }
}

impl ExecutableInstruction<Z80> for ADC_A_D {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::adc_r_r!(
            &mut cpu.registers.gp[0].a,
            cpu.registers.gp[0].d,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("8a.json");
    test_instruction_parse!(ADC_A_D);
}
