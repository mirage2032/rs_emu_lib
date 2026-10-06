use std::fmt;
use std::fmt::Display;

use super::{common, name, opcode, repeat, step};
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// OUTI, OUTD, OTIR and OTDR: count B down, write (HL) to port BC, and step HL.
#[derive(Debug)]
pub struct OUT_BLOCK<const DOWN: bool, const REPEAT: bool> {
    common: InstructionCommon,
}

pub type OUTI = OUT_BLOCK<false, false>;
pub type OUTD = OUT_BLOCK<true, false>;
pub type OTIR = OUT_BLOCK<false, true>;
pub type OTDR = OUT_BLOCK<true, true>;

impl<const DOWN: bool, const REPEAT: bool> OUT_BLOCK<DOWN, REPEAT> {
    pub fn new() -> Self {
        Self { common: common() }
    }
}

impl<const DOWN: bool, const REPEAT: bool> Display for OUT_BLOCK<DOWN, REPEAT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(name(["OUTI", "OUTD", "OTIR", "OTDR"], DOWN, REPEAT))
    }
}

impl<const DOWN: bool, const REPEAT: bool> BaseInstruction for OUT_BLOCK<DOWN, REPEAT> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, opcode(0xA3, DOWN, REPEAT)]
    }
}

impl<const DOWN: bool, const REPEAT: bool> ExecutableInstruction<Z80> for OUT_BLOCK<DOWN, REPEAT> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp;
        let value = memory.read_8(gp.hl)?;
        gp.b = gp.b.wrapping_sub(1);
        io.write(gp.bc, value)?; // the port is BC after B counts down
        gp.hl = step(gp.hl, DOWN);
        let (b, l) = (gp.b, gp.l);
        alu::block_io(&mut gp.f, value, value as u16 + l as u16, b);
        let again = REPEAT && b != 0;
        if again {
            alu::block_io_repeat(&mut cpu.registers.gp.f, cpu.registers.pc, value, b);
        }
        repeat(&mut self.common, again);
        Ok(())
    }
}
