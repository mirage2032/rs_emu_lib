use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct EX_AF_SAF {
    common: InstructionCommon,
}

impl EX_AF_SAF {
    pub fn new() -> EX_AF_SAF {
        EX_AF_SAF {
            common: InstructionCommon::new(1, 4, true),
        }
    }
}

impl Display for EX_AF_SAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX af, af'")
    }
}

impl BaseInstruction for EX_AF_SAF {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x08]
    }
}

impl ExecutableInstruction<Z80> for EX_AF_SAF {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let af = cpu.registers.main.af;
        cpu.registers.main.af = cpu.registers.shadow.af;
        cpu.registers.shadow.af = af;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    test_instruction_parse!(EX_AF_SAF);
}
