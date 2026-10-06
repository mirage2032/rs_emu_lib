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
pub struct LD_IDX_PNN<R: IndexRegister> {
    common: InstructionCommon,
    nn: u16,
    index: PhantomData<R>,
}

pub type LD_IX_PNN = LD_IDX_PNN<IX>;
pub type LD_IY_PNN = LD_IDX_PNN<IY>;

impl<R: IndexRegister> LD_IDX_PNN<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        Ok(Self {
            common: InstructionCommon::new(4, 20, true),
            nn: memory.read_16(pos.wrapping_add(2))?,
            index: PhantomData,
        })
    }

    pub fn new_with_value(nn: u16) -> Self {
        Self {
            common: InstructionCommon::new(4, 20, true),
            nn,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for LD_IDX_PNN<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD {name}, (0x{:04X})", self.nn, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for LD_IDX_PNN<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![R::PREFIX, 0x2A, nn_lsb[0], nn_lsb[1]]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for LD_IDX_PNN<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_16(self.nn)?;
        *R::get_mut(&mut cpu.registers) = val;
        Ok(())
    }
}
