use std::fmt::Display;

use crate::cpu::registers::BaseRegister;
use crate::cpu::Cpu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug, Clone, Copy)]
pub struct InstructionCommon {
    length: u16,
    cycles: u16,
    increment_pc: bool,
}

impl InstructionCommon {
    pub fn new(length: u16, cycles: u16, increment_pc: bool) -> InstructionCommon {
        InstructionCommon {
            length,
            cycles,
            increment_pc,
        }
    }
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
            cpu.registers_mut().pc += inst_length;
            // Increment r register
            match cpu.registers_mut().other.get_mut("r") {
                Some(BaseRegister::Bit8(r)) => {
                    *r = r.wrapping_add(1);
                }
                _ => {
                    panic!("r register is not 8 bit")
                }
            }
        }
        Ok(())
    }
}
//MACROS
//STACK PUSH/POP
macro_rules! push_8 {
    ($val:expr, $memory:expr, $sp:expr) => {
        $sp -= 1;
        $memory
            .write_8($sp, $val)
            .map_err(|_| "Error pushing value to stack")?;
    };
}
pub(crate) use push_8;

// #[macro_export]
macro_rules! push_16 {
    ($val:expr, $memory:expr, $sp:expr) => {
        $sp -= 2;
        $memory
            .write_16($sp, $val)
            .map_err(|_| "Error pushing value to stack")?;
    };
}
pub(crate) use push_16;

macro_rules! pop_8 {
    ($memory:expr, $sp:expr) => {
        let val = $memory
            .read_8(*sp)
            .map_err(|_| "Error popping value from stack")?;
        *$sp += 1;
        val
    };
}
pub(crate) use pop_8;

macro_rules! pop_16 {
    ($memory:expr, $sp:expr) => {
        let val = $memory
            .read_16(*sp)
            .map_err(|_| "Error popping value from stack")?;
        *$sp += 2;
        val
    };
}
pub(crate) use pop_16;