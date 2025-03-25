use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct XOR_PHL {
    common: InstructionCommon,
}

impl XOR_PHL {
    pub fn new() -> XOR_PHL {
        XOR_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for XOR_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XOR (HL)")
    }
}

impl BaseInstruction for XOR_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xAE]
    }
}

impl ExecutableInstruction<Z80> for XOR_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.gp.hl)?;
        super::xor_r_r_setf!(&mut cpu.registers.gp.a, &val, &mut cpu.registers.gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ae");
    test_instruction_parse!(XOR_PHL);
}
