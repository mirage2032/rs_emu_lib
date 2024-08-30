use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_PBC_A {
    common: InstructionCommon,
}

impl LD_PBC_A {
    pub fn new() -> LD_PBC_A {
        LD_PBC_A {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for LD_PBC_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (BC), A")
    }
}

impl BaseInstruction for LD_PBC_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x02]
    }
}

impl ExecutableInstruction<Z80> for LD_PBC_A {
    fn runner(&self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let location = cpu.registers.gp[0].bc;
        memory.write_8(location, cpu.registers.gp[0].a)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;

    test_instruction_parse!(LD_PBC_A);
}
