use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// RRD: rotates the three BCD digits in the low digit of A and the two of (HL) right
/// by one digit: A's low digit moves to (HL)'s high digit, that one to its low
/// digit, and (HL)'s low digit to A's low one.
#[derive(Debug)]
pub struct RRD {
    common: InstructionCommon,
}

impl RRD {
    pub fn new() -> RRD {
        RRD {
            common: InstructionCommon::new(2, 18, true),
        }
    }
}

impl Display for RRD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRD")
    }
}

impl BaseInstruction for RRD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, 0x67]
    }
}

impl ExecutableInstruction<Z80> for RRD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let hl = cpu.registers.gp.hl;
        let value = memory.read_8(hl)?;
        let a = cpu.registers.gp.a;
        memory.write_8(hl, (a << 4) | (value >> 4))?;
        let a = (a & 0xF0) | (value & 0x0F);
        cpu.registers.gp.a = a;
        alu::sz53p(&mut cpu.registers.gp.f, a);
        Ok(())
    }
}
