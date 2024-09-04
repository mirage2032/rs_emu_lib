use std::fmt;
use std::fmt::Display;

use once_cell::sync::Lazy;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(3, 13, true));

#[derive(Debug)]
pub struct LD_PNN_A {
    common: InstructionCommon,
    nn: u16,
}

impl LD_PNN_A {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<LD_PNN_A, String> {
        Ok(LD_PNN_A {
            common: *COMMON,
            nn: memory.read_16(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(nn: u16) -> LD_PNN_A {
        LD_PNN_A {
            common: *COMMON,
            nn,
        }
    }
}

impl Display for LD_PNN_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD (0x{:02x}), HL", self.nn)
    }
}

impl BaseInstruction for LD_PNN_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![0x32, nn_lsb[0], nn_lsb[1]]
    }
}

impl ExecutableInstruction<Z80> for LD_PNN_A {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        memory.write_8(self.nn, cpu.registers.gp[0].a)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("32.json");
    test_instruction_parse!(LD_PNN_A, [0xbeef]);
}
