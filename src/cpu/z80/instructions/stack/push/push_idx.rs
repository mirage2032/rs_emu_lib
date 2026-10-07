use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::push_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct PUSH_IDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type PUSH_IX = PUSH_IDX<IX>;
pub type PUSH_IY = PUSH_IDX<IY>;

impl<R: IndexRegister> PUSH_IDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 15, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for PUSH_IDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH {name}", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for PUSH_IDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0xe5]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for PUSH_IDX<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        push_16!(R::get(&cpu.registers), memory, cpu.registers.sp);

        Ok(())
    }
}
