use std::fmt;
use std::fmt::Display;

use super::{common, name, opcode, repeat, step};
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// CPI, CPD, CPIR and CPDR: compare A with (HL), step HL, and count BC down. The
/// repeating forms stop when BC reaches 0 or A matches.
#[derive(Debug)]
pub struct CP_BLOCK<const DOWN: bool, const REPEAT: bool> {
    common: InstructionCommon,
}

pub type CPI = CP_BLOCK<false, false>;
pub type CPD = CP_BLOCK<true, false>;
pub type CPIR = CP_BLOCK<false, true>;
pub type CPDR = CP_BLOCK<true, true>;

impl<const DOWN: bool, const REPEAT: bool> CP_BLOCK<DOWN, REPEAT> {
    pub fn new() -> Self {
        Self { common: common() }
    }
}

impl<const DOWN: bool, const REPEAT: bool> Display for CP_BLOCK<DOWN, REPEAT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(name(["CPI", "CPD", "CPIR", "CPDR"], DOWN, REPEAT))
    }
}

impl<const DOWN: bool, const REPEAT: bool> BaseInstruction for CP_BLOCK<DOWN, REPEAT> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, opcode(0xA1, DOWN, REPEAT)]
    }
}

impl<const DOWN: bool, const REPEAT: bool> ExecutableInstruction<Z80> for CP_BLOCK<DOWN, REPEAT> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp;
        let value = memory.read_8(gp.hl)?;
        gp.hl = step(gp.hl, DOWN);
        gp.bc = gp.bc.wrapping_sub(1);
        let (a, bc) = (gp.a, gp.bc);
        alu::block_cp(&mut gp.f, a, value, bc);
        let again = REPEAT && bc != 0 && !gp.f.zero();
        if again {
            alu::block_repeat(&mut cpu.registers.gp.f, cpu.registers.pc);
        }
        repeat(&mut self.common, again);
        Ok(())
    }
}
