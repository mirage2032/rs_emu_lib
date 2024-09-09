use std::collections::HashMap;

use crate::cpu::Cpu;
use crate::cpu::instruction::{ExecutableInstruction, InstructionParser, push_16};
use crate::cpu::registers::{AllMutRegisters, AllRegisters, GPByteRegisters};
use crate::io::{InterruptType, IO};

use super::super::memory::{memdevices::ROM, Memory, MemoryDevice};

pub mod instructions;
pub mod parser;

#[cfg(test)]
mod test;

pub struct Z80Registers {
    pub gp: GPByteRegisters,
    pub gp_alt: GPByteRegisters,
    pub ix: u16,
    pub iy: u16,
    pub i: u8,
    pub r: u8,
    pub sp: u16,
    pub pc: u16,
}
impl Z80Registers{
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.gp, &mut self.gp_alt);
    }
}
impl Default for Z80Registers {
    fn default() -> Self {
        Z80Registers {
            gp: GPByteRegisters::default(),
            gp_alt: GPByteRegisters::default(),
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            sp: 0xFFFF,
            pc: 0,
        }
    }
}
pub struct Z80 {
    registers: Z80Registers,
    parser: parser::Z80Parser,
    halted: bool,
}

impl Default for Z80 {
    fn default() -> Self {
        Z80 {
            registers: Z80Registers::default(),
            parser: parser::Z80Parser::default(),
            halted: false,
        }
    }
}

impl Z80 {
    fn handle_interrupt(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Option<Box<dyn ExecutableInstruction<Z80>>>, String> {
        match io.get_interrupt() {
            Some((int_vector, id)) => {
                let ret_instr: Option<Box<dyn ExecutableInstruction<Z80>>> = match int_vector {
                    InterruptType::NMI => {
                        self.registers.pc = 0x66;
                        None
                    }
                    InterruptType::IM0(instruction) => {
                        let rom: ROM = vec![instruction].into();
                        let instruction = parser::Z80Parser::from_memdev(&rom, 0)?;
                        Some(instruction)
                    }
                    remaining => {
                        push_16!(self.registers.pc, memory, self.registers.sp);
                        // self.registers.swap(); // TODO: Check if registers are swapped automatically
                        match remaining {
                            InterruptType::IM1 => {
                                self.registers.pc = 0x38;
                            }
                            InterruptType::IM2(int_vector) => {
                                self.registers.pc = u16::from_le_bytes([int_vector, self.registers.i]);
                            }
                            _ => unreachable!("IM0/NMI should have been handled"),
                        }
                        None
                    }
                };
                io.ack_int(id)?;
                Ok(ret_instr)
            }
            None => Ok(None),
        }
    }
}

impl Cpu for Z80 {
    fn step(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String> {
        let res = self.handle_interrupt(memory, io)?; // If IM1 interrupt it will be returned and executed
        let mut instruction: Box<dyn ExecutableInstruction<Z80>> = match res {
            Some(instruction) => instruction,
            None => parser::Z80Parser::from_memdev(memory, self.registers.pc)?,
        };
        // println!("Executing: {:?}", self.registers.gp[0].f);
        // println!("HL: {:X},BC:{:X}", self.registers.gp[0].hl,self.registers.gp[0].bc);
        instruction.execute(memory, self, io)?;
        let common = instruction.common();
        self.registers.r = self.registers.r.wrapping_add(1) % 0x80;
        if common.increment_pc {
            let inst_length = common.length;
            let new_pc = self.registers.pc.wrapping_add(inst_length);
            self.registers.pc = new_pc;
        }
        // println!("Executing: {:?}", self.registers.gp[0].f);
        Ok(instruction)
    }
    fn parser(&self) -> &dyn InstructionParser<Z80> {
        &self.parser
    }

    fn registers(&self) -> AllRegisters {
        let mut other8bit = HashMap::new();
        let mut other16bit = HashMap::new();
        other16bit.insert("ix", &self.registers.ix);
        other16bit.insert("iy", &self.registers.iy);
        other8bit.insert("i", &self.registers.i);
        other8bit.insert("r", &self.registers.r);
        AllRegisters {
            gp: vec![&self.registers.gp, &self.registers.gp_alt],
            other8bit,
            other16bit,
            sp: &self.registers.sp,
            pc: &self.registers.pc,
        }
    }
    fn registers_mut(&mut self) -> AllMutRegisters {
        let mut other8bit = HashMap::new();
        let mut other16bit = HashMap::new();
        other16bit.insert("ix", &mut self.registers.ix);
        other16bit.insert("iy", &mut self.registers.iy);
        other8bit.insert("i", &mut self.registers.i);
        other8bit.insert("r", &mut self.registers.r);
        AllMutRegisters {
            gp: vec![&mut self.registers.gp, &mut self.registers.gp_alt],
            other8bit,
            other16bit,
            sp: &mut self.registers.sp,
            pc: &mut self.registers.pc,
        }
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}
