use super::registers::RegisterOps;
use super::super::memory::Memory;

pub struct I8080 {}

impl I8080 {
    pub fn new() -> I8080 {
        I8080 {}
    }
}

impl super::super::cpu::Cpu for I8080 {
    fn step(&mut self, memory: &mut Memory) -> u16 {
        0
    }
    fn registers(&mut self) -> &mut dyn RegisterOps {
        unimplemented!()
    }

    fn decode(&self, memory: &Memory, pos: u16) -> Box<dyn super::super::cpu::BaseInstruction> {
        unimplemented!()
    }
}