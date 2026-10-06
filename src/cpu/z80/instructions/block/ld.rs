use std::fmt;
use std::fmt::Display;

use super::{common, name, opcode, repeat, step};
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// LDI, LDD, LDIR and LDDR: copy (HL) to (DE), step HL and DE, and count BC down.
#[derive(Debug)]
pub struct LD_BLOCK<const DOWN: bool, const REPEAT: bool> {
    common: InstructionCommon,
}

pub type LDI = LD_BLOCK<false, false>;
pub type LDD = LD_BLOCK<true, false>;
pub type LDIR = LD_BLOCK<false, true>;
pub type LDDR = LD_BLOCK<true, true>;

impl<const DOWN: bool, const REPEAT: bool> LD_BLOCK<DOWN, REPEAT> {
    pub fn new() -> Self {
        Self { common: common() }
    }
}

impl<const DOWN: bool, const REPEAT: bool> Display for LD_BLOCK<DOWN, REPEAT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(name(["LDI", "LDD", "LDIR", "LDDR"], DOWN, REPEAT))
    }
}

impl<const DOWN: bool, const REPEAT: bool> BaseInstruction for LD_BLOCK<DOWN, REPEAT> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, opcode(0xA0, DOWN, REPEAT)]
    }
}

impl<const DOWN: bool, const REPEAT: bool> ExecutableInstruction<Z80> for LD_BLOCK<DOWN, REPEAT> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp;
        let value = memory.read_8(gp.hl)?;
        memory.write_8(gp.de, value)?;
        gp.hl = step(gp.hl, DOWN);
        gp.de = step(gp.de, DOWN);
        gp.bc = gp.bc.wrapping_sub(1);
        let (a, bc) = (gp.a, gp.bc);
        alu::block_ld(&mut gp.f, a, value, bc);
        let again = REPEAT && bc != 0;
        if again {
            alu::block_repeat(&mut cpu.registers.gp.f, cpu.registers.pc);
        }
        repeat(&mut self.common, again);
        Ok(())
    }
}
