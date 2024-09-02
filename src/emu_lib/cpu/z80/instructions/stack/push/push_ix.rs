use crate::cpu::instruction::push_16;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};
use std::fmt;
use std::fmt::Display;
use crate::cpu::registers::BaseRegister;

#[derive(Debug)]
pub struct PUSH_IX {
    common: InstructionCommon,
}

impl PUSH_IX {
    pub fn new() -> PUSH_IX {
        PUSH_IX {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for PUSH_IX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH IX")
    }
}

impl BaseInstruction for PUSH_IX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xe5]
    }
}

impl ExecutableInstruction<Z80> for PUSH_IX {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(val)) => { push_16!(*val, memory, cpu.registers.sp); },
            _ => return Err("Invalid register".to_string()),
        }
        match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => { *val = val.wrapping_add(1) % 128; },
            _ => return Err("Invalid register".to_string()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("dd e5.json");
    test_instruction_parse!(PUSH_IX);
}
