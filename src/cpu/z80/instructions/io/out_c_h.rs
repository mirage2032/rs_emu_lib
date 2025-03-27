use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct OUT_C_H {
    common: InstructionCommon,
}

impl OUT_C_H {
    pub fn new() -> OUT_C_H {
        OUT_C_H {
            common: InstructionCommon::new(2, 12, true),
        }
    }
}

impl Display for OUT_C_H {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OUT (C), H")
    }
}

impl BaseInstruction for OUT_C_H {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed,0x61]
    }
}

impl ExecutableInstruction<Z80> for OUT_C_H {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        io.write(cpu.registers.gp.c, cpu.registers.gp.h)?;
        cpu.registers.r = cpu.registers.r.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed 61");
    test_instruction_parse!(OUT_C_H);
}
