use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ADD_IDX_BC<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type ADD_IX_BC = ADD_IDX_BC<IX>;
pub type ADD_IY_BC = ADD_IDX_BC<IY>;

impl<R: IndexRegister> ADD_IDX_BC<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 15, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for ADD_IDX_BC<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD {name}, BC", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for ADD_IDX_BC<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x09]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for ADD_IDX_BC<R> {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let (a, b) = (R::get(&cpu.registers), cpu.registers.gp.bc);
        *R::get_mut(&mut cpu.registers) = alu::add16(&mut cpu.registers.gp.f, a, b);
        Ok(())
    }
}
