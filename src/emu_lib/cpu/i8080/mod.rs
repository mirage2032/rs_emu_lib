use crate::emu_lib::cpu::{BaseInstruction, Cpu, CPUType, ExecutableInstruction, InstructionDecoder, InstructionEncoder, RegisterOps};
use crate::emu_lib::memory::ReadableMemory;

use super::super::memory::Memory;

pub struct I8080 {
    halted: bool,
}

impl I8080 {
    pub fn new() -> I8080 {
        I8080 { halted: false }
    }
}

impl InstructionDecoder for I8080 {
    fn decode(_: &impl ReadableMemory, _: u16) -> Result<Box<dyn ExecutableInstruction<Self>>, String> {
        unimplemented!()
    }
}

impl InstructionEncoder for I8080 {
    fn encode(_: String) -> Result<Box<dyn ExecutableInstruction<Self>>, String> {
        unimplemented!()
    }
}

impl Cpu for I8080 {
    fn step(&mut self, _: &mut Memory) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }

    fn encode(&self, _: String) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }

    fn decode_mem(&self, _: &Memory, _: u16) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }

    fn decode_vec(&self, _: &Vec<u8>) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }

    fn type_of(&self) -> CPUType {
        CPUType::I8080
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