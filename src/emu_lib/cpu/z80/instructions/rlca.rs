use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct RLCA {
    common: InstructionCommon,
}

impl RLCA {
    pub fn new() -> RLCA {
        RLCA {
            common: InstructionCommon {
                length: 1,
                cycles: 4,
                increment_pc: true,
            },
        }
    }
}

impl Display for RLCA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rlca")
    }
}

impl BaseInstruction for RLCA {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x07]
    }
}

impl ExecutableInstruction<Z80> for RLCA {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let carry = cpu.registers.main.a >> 7;
        cpu.registers.main.f.set_carry(carry != 0);
        let a = (cpu.registers.main.a << 1) | carry;
        cpu.registers.main.a = a;
        cpu.registers.main.f.set_add_sub(false);
        cpu.registers.main.f.set_half_carry(false);
        Ok(())
    }
}