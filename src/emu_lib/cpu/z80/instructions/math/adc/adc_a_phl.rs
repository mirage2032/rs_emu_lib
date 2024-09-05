use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct ADC_A_PHL {
    common: InstructionCommon,
}

impl ADC_A_PHL {
    pub fn new() -> ADC_A_PHL {
        ADC_A_PHL {
            common: InstructionCommon::new(1, 7, true)
        }
    }
}

impl Display for ADC_A_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD A, (HL)",)
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
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.gp[0].hl)?;
        super::adc_r_r_setf!(&mut cpu.registers.gp[0].a, val, cpu.registers.gp[0].f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("8e");
    test_instruction_parse!(ADC_A_PHL);
}
