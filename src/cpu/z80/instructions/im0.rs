use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM0 {
    common: InstructionCommon,
    opcode: u8,
}

impl IM0 {
    pub fn new() -> IM0 {
        IM0::with_opcode(0x46)
    }

    /// IM 0 encoded as `ED opcode`: 0x46, or the undocumented 0x4E, 0x66 or 0x6E.
    pub fn with_opcode(opcode: u8) -> IM0 {
        debug_assert!(matches!(opcode, 0x46 | 0x4E | 0x66 | 0x6E), "IM 0 isn't ED {opcode:02X}");
        IM0 {
            common: InstructionCommon::new(2, 8, true),
            opcode,
        }
    }
}

impl Display for IM0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM 0",)
    }
}

impl BaseInstruction for IM0 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, self.opcode]
    }
}

impl ExecutableInstruction<Z80> for IM0 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.interrupts.im = 0;
        Ok(())
    }
}
