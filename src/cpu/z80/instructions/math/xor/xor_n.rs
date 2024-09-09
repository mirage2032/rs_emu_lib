use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct XOR_N {
    common: InstructionCommon,
    n: u8,
}

impl XOR_N {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<XOR_N, String> {
        Ok(XOR_N {
            common: InstructionCommon::new(2, 7, true),
            n: memory.read_8(pos.wrapping_add(1))?,
        })
    }

    pub fn new_with_value(n: u8) -> XOR_N {
        XOR_N {
            common: InstructionCommon::new(2, 7, true),
            n,
        }
    }
}

impl Display for XOR_N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XOR 0x{:02x}", self.n)
    }
}

impl BaseInstruction for XOR_N {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xEE, self.n]
    }
}

impl ExecutableInstruction<Z80> for XOR_N {
    fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        super::xor_r_r_setf!(
            &mut cpu.registers.gp.a,
            &self.n,
            &mut cpu.registers.gp.f
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ee");
    test_instruction_parse!(XOR_N, [0xbf]);
}
