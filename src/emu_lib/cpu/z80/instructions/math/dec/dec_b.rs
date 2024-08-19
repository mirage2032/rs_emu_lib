use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct DEC_B {
    common: InstructionCommon,
}

impl DEC_B {
    pub fn new() -> DEC_B {
        DEC_B {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for DEC_B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC B")
    }
}

impl BaseInstruction for DEC_B {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x05]
    }
}

impl ExecutableInstruction<Z80> for DEC_B {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp[0];
        super::dec_r!(&mut gp.b, &mut gp.f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;

    test_instruction_parse!(DEC_B);
}
