use std::fmt;
use std::fmt::Display;

use super::{common, name, opcode, repeat, step};
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

/// INI, IND, INIR and INDR: read port BC into (HL), step HL, and count B down.
#[derive(Debug)]
pub struct IN_BLOCK<const DOWN: bool, const REPEAT: bool> {
    common: InstructionCommon,
}

pub type INI = IN_BLOCK<false, false>;
pub type IND = IN_BLOCK<true, false>;
pub type INIR = IN_BLOCK<false, true>;
pub type INDR = IN_BLOCK<true, true>;

impl<const DOWN: bool, const REPEAT: bool> IN_BLOCK<DOWN, REPEAT> {
    pub fn new() -> Self {
        Self { common: common() }
    }
}

impl<const DOWN: bool, const REPEAT: bool> Display for IN_BLOCK<DOWN, REPEAT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(name(["INI", "IND", "INIR", "INDR"], DOWN, REPEAT))
    }
}

impl<const DOWN: bool, const REPEAT: bool> BaseInstruction for IN_BLOCK<DOWN, REPEAT> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xED, opcode(0xA2, DOWN, REPEAT)]
    }
}

impl<const DOWN: bool, const REPEAT: bool> ExecutableInstruction<Z80> for IN_BLOCK<DOWN, REPEAT> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        let gp = &mut cpu.registers.gp;
        let value = io.read(gp.bc)?; // the port is BC before B counts down
        memory.write_8(gp.hl, value)?;
        gp.hl = step(gp.hl, DOWN);
        gp.b = gp.b.wrapping_sub(1);
        let c = if DOWN { gp.c.wrapping_sub(1) } else { gp.c.wrapping_add(1) };
        let b = gp.b;
        alu::block_io(&mut gp.f, value, value as u16 + c as u16, b);
        let again = REPEAT && b != 0;
        if again {
            alu::block_io_repeat(&mut cpu.registers.gp.f, cpu.registers.pc, value, b);
        }
        repeat(&mut self.common, again);
        Ok(())
    }
}
