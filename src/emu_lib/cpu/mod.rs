use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::emu_lib::memory::{Memory, ReadableMemory};
use crate::io::IO;

pub mod z80;
pub mod i8080;

#[derive(PartialEq, Copy, Clone)]
pub enum CPUType {
    Z80,
    I8080,
}

pub enum SingleRegister {
    Bit8(u8),
    Bit16(u16),
}

pub trait RegisterOps: Debug {
    fn clear(&mut self);
    fn set_8(&mut self, register: &str, value: u8);
    fn set_16(&mut self, register: &str, value: u16);
    fn get_8(&self, register: &str) -> u8;
    fn get_16(&self, register: &str) -> u16;
    fn get_all(&self) -> HashMap<&str, SingleRegister>;
    fn pc(&self) -> &u16;
    fn pc_mut(&mut self) -> &mut u16;
}

pub trait InstructionDecoder {
    fn decode(memory: &impl ReadableMemory, pos: u16) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String>;
}

pub trait InstructionEncoder {
    fn encode(instruction: String) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String>;
}

pub trait Cpu: Send {
    fn step(&mut self, memory: &mut Memory, io: &mut IO) -> Result<Box<(dyn BaseInstruction)>, String>;
    fn encode(&self, instruction: String) -> Result<Box<(dyn BaseInstruction)>, String>;
    fn decode_mem(&self, memory: &Memory, pos: u16) -> Result<Box<(dyn BaseInstruction)>, String>;
    fn decode_vec(&self, vec: &Vec<u8>) -> Result<Box<(dyn BaseInstruction)>, String>;
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

impl InstructionCommon {
    pub fn get_length(&self) -> u16 {
        self.length
    }
    pub fn get_cycles(&self) -> u16 {
        self.cycles
    }
    pub fn get_increment_pc(&self) -> bool {
        self.increment_pc
    }
}

pub trait BaseInstruction: Display {
    fn common(&self) -> &InstructionCommon;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ExecutableInstruction<T: Cpu>: BaseInstruction {
    fn runner(&self, memory: &mut Memory, cpu: &mut T, io: &mut IO) -> Result<(), String>;
    fn execute(&self, memory: &mut Memory, cpu: &mut T, io: &mut IO) -> Result<(), String> {
        self.runner(memory, cpu, io)?;
        if self.common().increment_pc {
            let inst_length = self.common().length;
            *cpu.registers().pc_mut() += inst_length;
            let new_r = cpu.registers().get_8("r").wrapping_add(inst_length as u8);
            cpu.registers().set_8("r", new_r);
        }
        Ok(())
    }
}