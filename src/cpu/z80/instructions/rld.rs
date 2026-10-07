use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// RLD: rotates the three BCD digits in the low digit of A and the two of (HL) left
/// by one digit: (HL)'s low digit moves to its high one, its high digit to A's low
/// digit, and A's low digit to (HL)'s low one.
#[derive(Debug)]
pub struct RLD {
    common: InstructionCommon,
}

impl RLD {
    pub fn new() -> RLD {
        RLD {
            common: InstructionCommon::new(2, 18, true),
        }
    }
}

impl Display for RLD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLD")
    }
}

impl BaseInstruction for RLD {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, 0x6f]
    }
}

impl ExecutableInstruction<Z80> for RLD {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let hl = cpu.registers.gp.hl;
        let value = memory.read_8(hl)?;
        let a = cpu.registers.gp.a;
        memory.write_8(hl, (value << 4) | (a & 0x0F))?;
        let a = (a & 0xF0) | (value >> 4);
        cpu.registers.gp.a = a;
        alu::sz53p(&mut cpu.registers.gp.f, a);
        Ok(())
    }
}
