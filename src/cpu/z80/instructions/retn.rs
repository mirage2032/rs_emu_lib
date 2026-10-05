use crate::memory::MemoryDevice;
use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{pop_16, BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct RETN {
    common: InstructionCommon,
    opcode: u8,
}

impl RETN {
    pub fn new() -> RETN {
        RETN::with_opcode(0x45)
    }

    /// RETN encoded as `ED opcode`: 0x45, or the undocumented 0x55, 0x5D, 0x65,
    /// 0x6D, 0x75 or 0x7D.
    pub fn with_opcode(opcode: u8) -> RETN {
        debug_assert!(
            matches!(opcode, 0x45 | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D),
            "RETN isn't ED {opcode:02X}"
        );
        RETN {
            common: InstructionCommon::new(2, 14, false),
            opcode,
        }
    }
}

impl Display for RETN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RETN",)
    }
}

impl BaseInstruction for RETN {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed, self.opcode]
    }
}

impl ExecutableInstruction<Z80> for RETN {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = pop_16!(memory, cpu.registers.sp);
        cpu.registers.inc_r();
        cpu.interrupts.iff1 = cpu.interrupts.iff2;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("ed", "45");
    test_instruction_parse!(RETN);

    // The undocumented mirrors behave the same.
    mod ed_55 {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "55");
    }

    mod ed_5d {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "5d");
    }

    mod ed_65 {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "65");
    }

    mod ed_6d {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "6d");
    }

    mod ed_75 {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "75");
    }

    mod ed_7d {
        use crate::cpu::z80::test::*;

        test_z80!("ed", "7d");
    }
}
