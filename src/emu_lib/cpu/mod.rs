use std::collections::HashMap;
use std::fmt::Display;

use crate::emu_lib::memory::Memory;

pub mod z80;
pub mod i8080;

#[derive(PartialEq)]
pub enum CPUType {
    Z80,
    I8080,
}

pub enum SingleRegister {
    Bit8(u8),
    Bit16(u16),
}

type Stack<T> = Vec<T>;

pub trait RegisterOps {
    fn clear(&mut self);
    fn set_8(&mut self, register: &str, value: u8);
    fn set_16(&mut self, register: &str, value: u16);
    fn get_all(&self) -> HashMap<&str, SingleRegister>;
    fn pc(&mut self) -> &mut u16;
    fn sp(&mut self) -> &mut Stack<u16>;
}

pub trait Cpu {
    fn step(&mut self, memory: &mut Memory) -> Result<u16, String>;
    fn decode(&self, memory: &Memory, pos: u16) -> Result<Box<(dyn BaseInstruction)>,String>;
    fn type_of(&self) -> CPUType;
    fn registers(&mut self) -> &mut dyn RegisterOps;
    fn halted(&self) -> bool;
    fn set_halted(&mut self, halted: bool);
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
    fn runner(&self, memory: &mut Memory, cpu: &mut T) -> Result<(), String>;
    fn execute(&self, memory: &mut Memory, cpu: &mut T) -> Result<(), String> {
        self.runner(memory, cpu)?;
        if self.common().increment_pc {
            *cpu.registers().pc() += self.common().length;
        }
        Ok(())
    }
}