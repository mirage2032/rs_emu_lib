use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(2, 7, true));

#[derive(Debug)]
pub struct LD_D_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_D_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_D_N, String> {
        Ok(LD_D_N {
            common: *COMMON,
            n: memory.read_8(pos + 1)?,
        })
    }

    pub fn new_with_value(n: u8) -> LD_D_N {
        LD_D_N { common: *COMMON, n }
    }
}

impl Display for LD_D_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD D, 0x{:02x}", self.n)
    }
}

impl BaseInstruction for LD_D_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x16, self.n]
    }
}

impl ExecutableInstruction<Z80> for LD_D_N {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.gp[0].d = self.n;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("16.json");
    test_instruction_parse!(LD_D_N, [0xbf]);
}
