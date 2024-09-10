use instruction::{BaseInstruction, InstructionParser};

use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::registers::{AllMutRegisters, AllRegisters};
use crate::io::IO;
use crate::memory::Memory;

pub mod i8080;
pub mod instruction;
pub mod registers;
#[cfg(test)]
pub mod test;
pub mod z80;

pub trait Cpu: Send + Default {
    fn step(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String>;
    fn parser(&self) -> &dyn InstructionParser<Self>;
    fn registers(&self) -> AllRegisters;
    fn registers_mut(&mut self) -> AllMutRegisters;

    fn pc(&self) -> u16;
    fn halted(&self) -> bool;
    fn set_halted(&mut self, halted: bool);
}
