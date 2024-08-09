use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct DEC_C {
    common: InstructionCommon,
}

impl DEC_C {
    pub fn new() -> DEC_C {
        DEC_C {
            common: InstructionCommon::new(1, 4, false),
        }
    }
}

impl Display for DEC_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC c")
    }
}

impl BaseInstruction for DEC_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x0d]
    }
}

impl ExecutableInstruction<Z80> for DEC_C {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::dec_r!(&mut cpu.registers.main.c, &mut cpu.registers.main.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;

    test_instruction_parse!(DEC_C);
}
