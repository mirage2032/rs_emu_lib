use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct SUB_PIDXD<R: IndexRegister> {
    common: InstructionCommon,
    d: i8,
    index: PhantomData<R>,
}

pub type SUB_PIXD = SUB_PIDXD<IX>;
pub type SUB_PIYD = SUB_PIDXD<IY>;

impl<R: IndexRegister> SUB_PIDXD<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        Ok(Self::new_with_value(memory.read_8(pos.wrapping_add(2))?))
    }

    pub fn new_with_value(d: u8) -> Self {
        Self {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for SUB_PIDXD<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUB ({name}+0x{:02X})", self.d, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for SUB_PIDXD<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x96, self.d as u8]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for SUB_PIDXD<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_8(R::get(&cpu.registers).wrapping_add(self.d as u16))?;
        cpu.registers.gp.a = alu::sub8(&mut cpu.registers.gp.f, cpu.registers.gp.a, val, false);
        Ok(())
    }
}
