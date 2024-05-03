use std::fmt;
use std::fmt::Display;

use crate::emu_lib::cpu::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub struct INC_BC {
    common: InstructionCommon,
}

impl INC_BC {
    pub fn new() -> INC_BC {
        INC_BC {
            common: InstructionCommon {
                length: 1,
                cycles: 6,
                increment_pc: true,
            },
        }
    }
}

impl Display for INC_BC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inc bc")
    }
}

impl BaseInstruction for INC_BC {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x03]
    }
}

impl ExecutableInstruction<Z80> for INC_BC {
    fn runner(&self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.main.bc = cpu.registers.main.bc.wrapping_add(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_instruction_test;

    generate_instruction_test!(INC_BC);
}