use instruction::BaseInstruction;
use registers::InstructionParser;

use crate::cpu::registers::AllRegisters;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub mod i8080;
pub mod instruction;
pub mod registers;
#[cfg(test)]
pub mod test;
pub mod z80;

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
    fn registers(&self) -> &AllRegisters;
    fn registers_mut(&mut self) -> &mut AllRegisters;
    fn halted(&self) -> bool;
    fn set_halted(&mut self, halted: bool);
}
