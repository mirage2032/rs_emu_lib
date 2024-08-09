use instruction::BaseInstruction;
use registers::InstructionParser;

use crate::cpu::registers::RegisterOps;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub mod i8080;
pub mod instruction;
pub mod registers;
pub mod z80;
#[cfg(test)]
pub mod test;

#[derive(PartialEq, Copy, Clone)]
pub enum CPUType {
    Z80,
    I8080,
}

pub trait Cpu: Send {
    fn step(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Box<(dyn BaseInstruction)>, String>;
    fn type_of(&self) -> CPUType;
    fn parser(&self) -> &dyn InstructionParser;
    fn registers(&self) -> &dyn RegisterOps;
    fn registers_mut(&mut self) -> &mut dyn RegisterOps;
    fn halted(&self) -> bool;
    fn set_halted(&mut self, halted: bool);
}
