use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::pop_16;
use crate::cpu::registers::BaseRegister;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct POP_IX {
    common: InstructionCommon,
}

impl POP_IX {
    pub fn new() -> POP_IX {
        POP_IX {
            common: InstructionCommon::new(2, 14, true),
        }
    }
}

impl Display for POP_IX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP IX")
    }
}

impl BaseInstruction for POP_IX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xdd, 0xe1]
    }
}

impl ExecutableInstruction<Z80> for POP_IX {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let result = pop_16!(memory, cpu.registers.sp);
        cpu.registers
            .other
            .insert("ix", BaseRegister::Bit16(result));
        match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => {
                *val = val.wrapping_add(1) % 128;
            }
            _ => return Err("Invalid register".to_string()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::emu_lib::cpu::test::test_instruction_parse;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("dd e1");
    test_instruction_parse!(POP_IX);
}
