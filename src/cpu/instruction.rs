#![allow(unused)]
use crate::cpu::Cpu;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use std::fmt::{Debug, Display};
use thiserror::Error;

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

pub trait BaseInstruction: Display + Debug + Send + Sync + 'static {
    fn common(&self) -> &InstructionCommon;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ExecutableInstruction<T: Cpu>: BaseInstruction {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut T, io: &mut IO) -> Result<(), String>;
}

//into error parse error
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Malformed instruction: {0}")]
    InvalidInstruction(String),
    #[error("Memory error: {0}")]
    MemoryError(#[from] MemoryReadError),
}

pub trait InstructionParser<T: Cpu> {
    // fn ins_from_mc_memory(
    //     memory: &Memory,
    //     pos: u16,
    // ) -> Result<Box<(dyn ExecutableInstruction<T>)>, ParseError>  where Self: Sized;
    // fn ins_from_mc_vec(
    //     memory: &Vec<u8>,
    //     pos: u16,
    // ) -> Result<Box<(dyn ExecutableInstruction<T>)>, ParseError>  where Self: Sized;
    fn ins_from_asm_string(
        &self,
        instruction: &str,
    ) -> Result<Box<(dyn ExecutableInstruction<T>)>, ParseError>;
    fn check_asm_lines(&self, lines: &[String]) -> Result<(), Vec<(u16, ParseError)>> {
        let mut errors = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            match self.ins_from_asm_string(line) {
                Ok(_) => (),
                Err(e) => errors.push((idx as u16, e)),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn ins_vec_from_asm_lines(
        &self,
        lines: &Vec<String>,
    ) -> Result<Vec<Box<(dyn ExecutableInstruction<T>)>>, ParseError> {
        let mut instructions = Vec::new();
        for line in lines {
            instructions.push(self.ins_from_asm_string(line)?);
        }
        Ok(instructions)
    }
    fn ins_from_machinecode(
        &self,
        memory: &dyn MemoryDevice,
        pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<T>)>, ParseError>;
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

use crate::memory::errors::MemoryReadError;
pub(crate) use pop_16;
