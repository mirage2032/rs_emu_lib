use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct XOR_PHL {
    common: InstructionCommon,
}

impl XOR_PHL {
    pub fn new() -> XOR_PHL {
        XOR_PHL {
            common: InstructionCommon::new(1, 7, true),
        }
    }
}

impl Display for XOR_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XOR (HL)")
    }
}

impl BaseInstruction for XOR_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xAE]
    }
}

impl ExecutableInstruction<Z80> for XOR_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(cpu.registers.gp.hl)?;
        cpu.registers.gp.a = alu::xor8(&mut cpu.registers.gp.f, cpu.registers.gp.a, val);
        Ok(())
    }
}
