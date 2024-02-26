use std::fmt::Display;

use crate::emu_lib::cpu::registers::RegisterOps;
use crate::emu_lib::memory::Memory;

pub mod z80;
pub mod i8080;
mod registers;

#[derive(PartialEq)]
pub enum CPUType {
    Z80,
    I8080,
}

pub trait Cpu {
    fn step(&mut self, memory: &mut Memory) -> u16;
    fn registers(&mut self) -> &mut dyn RegisterOps;
    fn decode(&self, memory: &Memory, pos: u16) -> Box<dyn BaseInstruction>;
}

pub struct InstructionCommon {
    length: u16,
    cycles: u16,
    increment_pc: bool,
}

pub trait BaseInstruction: Display {
    fn common(&self) -> &InstructionCommon;
}

trait ExecutableInstruction<T: Cpu>: BaseInstruction {
    fn runner(&self, memory: &mut Memory, cpu: &mut T);
    fn execute(&self, memory: &mut Memory, cpu: &mut T) {
        self.runner(memory, cpu);
        if self.common().increment_pc {
            cpu.registers().increment_pc(self.common().length);
        }
    }
}