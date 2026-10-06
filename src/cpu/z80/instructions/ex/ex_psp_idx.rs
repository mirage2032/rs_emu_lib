use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct EX_PSP_IDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type EX_PSP_IX = EX_PSP_IDX<IX>;
pub type EX_PSP_IY = EX_PSP_IDX<IY>;

impl<R: IndexRegister> EX_PSP_IDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 23, true),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for EX_PSP_IDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EX (SP), {name}", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for EX_PSP_IDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0xe3]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for EX_PSP_IDX<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let val = memory.read_16(cpu.registers.sp)?;
        memory.write_16(cpu.registers.sp, R::get(&cpu.registers))?;
        *R::get_mut(&mut cpu.registers) = val;
        Ok(())
    }
}
