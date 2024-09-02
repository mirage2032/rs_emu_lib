use std::fmt;
use std::fmt::Display;
use once_cell::sync::Lazy;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;
use crate::memory::MemoryDevice;

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(2, 7, true));

#[derive(Debug)]
pub struct ADC_A_N {
    common: InstructionCommon,
    n: u8,
}

impl ADC_A_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<ADC_A_N, String> {
        Ok(ADC_A_N {
            common: *COMMON,
            n: memory.read_8(pos + 1)?,
        })
    }
    
    pub fn new_with_value(n: u8) -> ADC_A_N {
        ADC_A_N {
            common: *COMMON,
            n,
        }
    }
}

impl Display for ADC_A_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC A, 0x{:02x}", self.n)
    }
}

impl BaseInstruction for ADC_A_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xCE, self.n]
    }
}

impl ExecutableInstruction<Z80> for ADC_A_N {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::adc_r_r!(
            &mut cpu.registers.gp[0].a,
            self.n,
            cpu.registers.gp[0].f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("ce.json");
    test_instruction_parse!(ADC_A_N, [0xce]);
}
