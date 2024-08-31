use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 7, true));

#[derive(Debug)]
pub struct LD_PDE_A {
    common: InstructionCommon,
}

impl LD_PDE_A {
    pub fn new() -> LD_PDE_A {
        LD_PDE_A { common: *COMMON }
    }
}

impl Display for LD_PDE_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (DE), A",)
    }
}

impl BaseInstruction for LD_PDE_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x12]
    }
}

impl ExecutableInstruction<Z80> for LD_PDE_A {
    fn runner(&self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let location = cpu.registers.gp[0].de;
        memory.write_8(location, cpu.registers.gp[0].a)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("12.json");
    test_instruction_parse!(LD_PDE_A);
}
