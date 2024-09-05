use std::fmt;
use std::fmt::Display;

use crate::cpu::z80::instructions::bit::rl::generics::rl_r_setf;
use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::{Memory, MemoryDevice};
use crate::cpu::z80::BaseRegister;

#[derive(Debug)]
pub struct RL_PHL {
    common: InstructionCommon,
}

impl RL_PHL {
    pub fn new() -> RL_PHL {
        RL_PHL {
            common: InstructionCommon::new(2, 15, true),
        }
    }
}

impl Display for RL_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRA (HL)")
    }
}

impl BaseInstruction for RL_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xcb, 0x1e]
    }
}

impl ExecutableInstruction<Z80> for RL_PHL {
    fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let mut value = memory.read_8(cpu.registers.gp[0].hl)?;
        rl_r_setf!(value, cpu.registers.gp[0].f);
        memory.write_8(cpu.registers.gp[0].hl, value)?;
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
    use crate::emu_lib::cpu::test::*;
    use crate::emu_lib::cpu::z80::test::*;

    test_z80!("cb", "16");
    test_instruction_parse!(RL_PHL);
}
