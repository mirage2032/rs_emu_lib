use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

#[derive(Debug)]
pub struct DEC_H {
    common: InstructionCommon,
}

impl DEC_H {
    pub fn new() -> DEC_H {
        DEC_H {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for DEC_H {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC H")
    }
}

impl BaseInstruction for DEC_H {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x25]
    }
}

impl ExecutableInstruction<Z80> for DEC_H {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::dec_r!(&mut gp.h, &mut gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("25.json");
    test_instruction_parse!(DEC_H);
}
