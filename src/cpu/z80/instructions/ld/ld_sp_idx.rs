use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_SP_IDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type LD_SP_IX = LD_SP_IDX<IX>;
pub type LD_SP_IY = LD_SP_IDX<IY>;

impl<R: IndexRegister> LD_SP_IDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 10, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for LD_SP_IDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD SP, {name}", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for LD_SP_IDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0xf9]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for LD_SP_IDX<R> {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.sp = R::get(&cpu.registers);
        Ok(())
    }
}
