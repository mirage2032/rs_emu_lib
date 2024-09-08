use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionParser};
use crate::cpu::registers::AllRegisters;
use crate::cpu::Cpu;
use crate::cpu::i8080::parser::I8080Parser;
use crate::io::IO;

use super::super::memory::Memory;
pub mod parser;

pub struct I8080 {
    halted: bool,
}

impl Default for I8080 {
    fn default() -> I8080 {
        I8080 { halted: false }
    }
}

impl Cpu for I8080 {
    fn step(
        &mut self,
        _: &mut Memory,
        _: &mut IO,
    ) -> Result<Box<dyn ExecutableInstruction<Self>>, String> {
        unimplemented!()
    }
    fn parser(&self) -> &dyn InstructionParser<Self> {
        unimplemented!()
    }

    fn registers(&self) -> &AllRegisters {
        unimplemented!()
    }
    fn registers_mut(&mut self) -> &mut AllRegisters {
        unimplemented!()
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}
