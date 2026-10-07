use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct DEC_IDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type DEC_IX = DEC_IDX<IX>;
pub type DEC_IY = DEC_IDX<IY>;

impl<R: IndexRegister> DEC_IDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 10, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for DEC_IDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC {name}", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for DEC_IDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x2b]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for DEC_IDX<R> {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        *R::get_mut(&mut cpu.registers) = R::get(&cpu.registers).wrapping_sub(1);
        Ok(())
    }
}
