use registers::Registers;

use crate::emu_lib::cpu::{BaseInstruction, Cpu};
use crate::emu_lib::cpu::z80::instructions::decode;

use super::registers::RegisterOps;
use super::super::memory::Memory;

mod registers;
mod instructions;

pub struct Z80 {
    registers: Registers,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            registers: Registers::new(),
        }
    }
}

impl Cpu for Z80 {
    fn step(&mut self, memory: &mut Memory) -> u16 {
        let instruction = decode(memory, self.registers.pc);
        instruction.execute(memory, self);
        instruction.common().cycles
    }

    fn registers(&mut self) -> &mut dyn RegisterOps {
        &mut self.registers
    }

    fn decode(&self, memory: &Memory, pos: u16) -> Box<dyn BaseInstruction> {
        decode(memory, pos)
    }
}