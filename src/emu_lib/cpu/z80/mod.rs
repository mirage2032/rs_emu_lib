use registers::Registers;

use crate::emu_lib::cpu::{BaseInstruction, Cpu, ExecutableInstruction};
use crate::emu_lib::cpu::z80::instructions::decode;

use super::super::memory::Memory;

mod registers;
mod instructions;

pub struct Z80 {
    registers: Registers,
    halted: bool,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            registers: Registers::new(),
            halted: false,
        }
    }
}

impl Cpu for Z80 {
    fn step(&mut self, memory: &mut Memory) -> Result<Box<(dyn BaseInstruction)>, String> {
        let instruction = decode(memory, self.registers.pc)?;
        instruction.execute(memory, self)?;
        Ok(instruction)
    }


    fn decode(&self, memory: &Memory, pos: u16) -> Result<Box<(dyn BaseInstruction)>, String> {
        decode(memory, pos).map(|i| i as Box<(dyn BaseInstruction)>)
    }
    fn type_of(&self) -> super::super::cpu::CPUType {
        super::super::cpu::CPUType::Z80
    }
    fn registers(&mut self) -> &mut dyn super::super::cpu::RegisterOps {
        &mut self.registers
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}