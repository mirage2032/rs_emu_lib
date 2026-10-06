use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IN_B_C {
    common: InstructionCommon,
}

impl IN_B_C {
    pub fn new() -> IN_B_C {
        IN_B_C {
            common: InstructionCommon::new(2, 12, true),
        }
    }
}

impl Display for IN_B_C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IN B, (C)")
    }
}

impl BaseInstruction for IN_B_C {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, 0x40]
    }
}

impl ExecutableInstruction<Z80> for IN_B_C {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let value = io.read(cpu.registers.gp.c)?;
        alu::sz53p(&mut cpu.registers.gp.f, value);
        cpu.registers.gp.b = value;
        Ok(())
    }
}
