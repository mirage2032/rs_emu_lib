use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct IM2 {
    common: InstructionCommon,
    opcode: u8,
}

impl IM2 {
    pub fn new() -> IM2 {
        IM2::with_opcode(0x5E)
    }

    /// IM 2 encoded as `ED opcode`: 0x5E, or the undocumented 0x7E.
    pub fn with_opcode(opcode: u8) -> IM2 {
        debug_assert!(matches!(opcode, 0x5E | 0x7E), "IM 2 isn't ED {opcode:02X}");
        IM2 {
            common: InstructionCommon::new(2, 8, true),
            opcode,
        }
    }
}

impl Display for IM2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IM 2",)
    }
}

impl BaseInstruction for IM2 {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, self.opcode]
    }
}

impl ExecutableInstruction<Z80> for IM2 {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        cpu.interrupts.im = 2;
        cpu.registers.inc_r();
        Ok(())
    }
}
