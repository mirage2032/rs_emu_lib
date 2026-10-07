use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::pop_16;
use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug)]
pub struct POP_IDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type POP_IX = POP_IDX<IX>;
pub type POP_IY = POP_IDX<IY>;

impl<R: IndexRegister> POP_IDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 14, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for POP_IDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP {name}", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for POP_IDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0xe1]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for POP_IDX<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let result = pop_16!(memory, cpu.registers.sp);
        *R::get_mut(&mut cpu.registers) = result;

        Ok(())
    }
}
