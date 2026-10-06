use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct LD_PIDXD_N<R: IndexRegister> {
    common: InstructionCommon,
    n: u8,
    d: i8,
    index: PhantomData<R>,
}

pub type LD_PIXD_N = LD_PIDXD_N<IX>;
pub type LD_PIYD_N = LD_PIDXD_N<IY>;

impl<R: IndexRegister> LD_PIDXD_N<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        let d = memory.read_8(pos.wrapping_add(2))?;
        let n = memory.read_8(pos.wrapping_add(3))?;
        Ok(Self::new_with_value(d, n))
    }

    pub fn new_with_value(d: u8, n: u8) -> Self {
        Self {
            common: InstructionCommon::new(4, 19, true),
            n,
            d: d as i8,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for LD_PIDXD_N<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD ({name}+0x{:02X}), 0x{:02X}", self.d, self.n, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for LD_PIDXD_N<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x36, self.d as u8, self.n]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for LD_PIDXD_N<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        memory.write_8(R::get(&cpu.registers).wrapping_add(self.d as u16), self.n)?;
        Ok(())
    }
}
