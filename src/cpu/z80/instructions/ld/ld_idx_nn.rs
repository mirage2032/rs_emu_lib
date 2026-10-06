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
pub struct LD_IDX_NN<R: IndexRegister> {
    common: InstructionCommon,
    nn: u16,
    index: PhantomData<R>,
}

pub type LD_IX_NN = LD_IDX_NN<IX>;
pub type LD_IY_NN = LD_IDX_NN<IY>;

impl<R: IndexRegister> LD_IDX_NN<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        Ok(Self::new_with_value(memory.read_16(pos.wrapping_add(2))?))
    }

    pub fn new_with_value(nn: u16) -> Self {
        Self {
            common: InstructionCommon::new(4, 14, true),
            nn,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for LD_IDX_NN<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD {name}, 0x{:04X}", self.nn, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for LD_IDX_NN<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        let nn_lsb = self.nn.to_le_bytes();
        vec![R::PREFIX, 0x21, nn_lsb[0], nn_lsb[1]]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for LD_IDX_NN<R> {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        *R::get_mut(&mut cpu.registers) = self.nn;
        Ok(())
    }
}
