use crate::emu_lib::cpu::{BaseInstruction, RegisterOps};

use super::super::memory::Memory;

pub struct I8080 {
    halted: bool,
}

impl I8080 {
    pub fn new() -> I8080 {
        I8080 { halted: false }
    }
}

impl super::super::cpu::Cpu for I8080 {
    fn step(&mut self, memory: &mut Memory) -> Result<u16, String> {
        unimplemented!()
    }

    fn decode(&self, memory: &Memory, pos: u16) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }

    fn type_of(&self) -> super::super::cpu::CPUType {
        super::super::cpu::CPUType::I8080
    }

    fn registers(&mut self) -> &mut dyn RegisterOps {
        unimplemented!()
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}