#![allow(unused)]
use std::fmt::{Debug, Display};

use crate::cpu::Cpu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug, Clone, Copy)]
pub struct InstructionCommon {
    pub length: u16,
    pub cycles: u16,
    pub increment_pc: bool,
}

impl InstructionCommon {
    pub fn new(length: u16, cycles: u16, increment_pc: bool) -> InstructionCommon {
        InstructionCommon {
            length,
            cycles,
            increment_pc,
        }
    }
}

pub trait BaseInstruction: Display + Debug {
    fn common(&self) -> &InstructionCommon;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ExecutableInstruction<T: Cpu>: BaseInstruction {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut T, io: &mut IO) -> Result<(), String>;
}

pub trait InstructionParser<T: Cpu> {
    fn ins_from_mem(
        &self,
        memory: &Memory,
        pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<T>)>, String>;
    fn ins_from_vec(
        &self,
        memory: &Vec<u8>,
        pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<T>)>, String>;
    fn ins_from_string(
        &self,
        instruction: &String,
    ) -> Result<Box<(dyn ExecutableInstruction<T>)>, String>;
    
}
//MACROS
//STACK PUSH/POP
macro_rules! push_8 {
    ($val:expr, $memory:expr, $sp:expr) => {
        $sp = $sp.wrapping_sub(1);
        $memory
            .write_8($sp, $val)
            .map_err(|_| "Error pushing value to stack")?;
    };
}

pub(crate) use push_8;

macro_rules! push_16 {
    ($val:expr, $memory:expr, $sp:expr) => {
        $sp = $sp.wrapping_sub(2);
        $memory
            .write_16($sp, $val)
            .map_err(|_| "Error pushing value to stack")?;
    };
}

pub(crate) use push_16;

macro_rules! pop_8 {
    ($memory:expr, $sp:expr) => {{
        let val = $memory
            .read_8($sp)
            .map_err(|_| "Error popping value from stack")?;
        $sp += $sp.wrapping_add(1);
        val
    }};
}

pub(crate) use pop_8;

macro_rules! pop_16 {
    ($memory:expr, $sp:expr) => {{
        let val = $memory
            .read_16($sp)
            .map_err(|_| "Error popping value from stack")?;
        $sp = $sp.wrapping_add(2);
        val
    }};
}

pub(crate) use pop_16;
