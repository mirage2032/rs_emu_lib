use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct DEC_PHL {
    common: InstructionCommon,
}

impl DEC_PHL {
    pub fn new() -> DEC_PHL {
        DEC_PHL {
            common: InstructionCommon::new(1, 11, true),
        }
    }
}

impl Display for DEC_PHL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC (HL)")
    }
}

impl BaseInstruction for DEC_PHL {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x34]
    }
}

impl ExecutableInstruction<Z80> for DEC_PHL {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let value_before = memory.read_8(cpu.registers.gp.hl)?;
        let result = value_before.wrapping_sub(1);
        memory.write_8(cpu.registers.gp.hl, result)?;
        // Update flags
        cpu.registers.gp.f.set_sign((result & (1 << 7)) != 0);
        cpu.registers.gp.f.set_parity_overflow(value_before == 0x80);
        cpu.registers
            .gp
            .f
            .set_half_carry((value_before & 0x0F) == 0);
        cpu.registers.gp.f.set_zero(result == 0);
        cpu.registers.gp.f.set_add_sub(true);

        // Set undocumented flags
        cpu.registers.gp.f.set_bit3((result >> 3) & 1 == 1);
        cpu.registers.gp.f.set_bit5((result >> 5) & 1 == 1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("35");
    test_instruction_parse!(DEC_PHL);
}
