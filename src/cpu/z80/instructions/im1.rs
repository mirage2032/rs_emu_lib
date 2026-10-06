use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM1 {
    common: InstructionCommon,
    opcode: u8,
}

impl IM1 {
    pub fn new() -> IM1 {
        IM1::with_opcode(0x56)
    }

    /// IM 1 encoded as `ED opcode`: 0x56, or the undocumented 0x76.
    pub fn with_opcode(opcode: u8) -> IM1 {
        debug_assert!(matches!(opcode, 0x56 | 0x76), "IM 1 isn't ED {opcode:02X}");
        IM1 {
            common: InstructionCommon::new(2, 8, true),
            opcode,
        }
    }
}

impl Display for IM1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM 1",)
    }
}

impl BaseInstruction for IM1 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, self.opcode]
    }
}

impl ExecutableInstruction<Z80> for IM1 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.interrupts.im = 1;
        Ok(())
    }
}
