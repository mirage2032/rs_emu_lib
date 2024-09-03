use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

const COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(2, 10, true));

#[derive(Debug)]
pub struct LD_PHL_N {
    common: InstructionCommon,
    n: u8,
}

impl LD_PHL_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_PHL_N, String> {
        Ok(LD_PHL_N {
            common: *COMMON,
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }
    pub fn new_with_value(n: u8) -> LD_PHL_N {
        LD_PHL_N { common: *COMMON, n }
    }
}

impl Display for LD_PHL_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (HL), 0x{:02x}", self.n)
    }
}

impl BaseInstruction for LD_PHL_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x36, self.n]
    }
}

impl ExecutableInstruction<Z80> for LD_PHL_N {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        memory.write_8(cpu.registers.gp[0].hl, self.n)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("36.json");
    test_instruction_parse!(LD_PHL_N, [0xe0]);
}
