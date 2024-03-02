use registers::Registers;

use crate::emu_lib::cpu::{BaseInstruction, Cpu, CPUType, InstructionDecoder, InstructionEncoder, RegisterOps};

use super::super::memory::Memory;

mod registers;
pub mod instructions;
mod encoder;
mod decoder;

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
        let instruction = Self::decode(memory, self.registers.pc)?;
        instruction.execute(memory, self)?;
        Ok(instruction)
    }

    fn encode(&self, instruction: String) -> Result<Box<(dyn BaseInstruction)>, String> {
        <Self as InstructionEncoder>::encode(instruction).map(|i| i as Box<(dyn BaseInstruction)>)
    }

    fn decode_mem(&self, memory: &Memory, pos: u16) -> Result<Box<(dyn BaseInstruction)>, String> {
        Self::decode(memory, pos).map(|i| i as Box<(dyn BaseInstruction)>)
    }

    fn decode_vec(&self, vec: &Vec<u8>) -> Result<Box<(dyn BaseInstruction)>, String> {
        Self::decode(vec, 0).map(|i| i as Box<(dyn BaseInstruction)>)
    }
    fn type_of(&self) -> CPUType {
        CPUType::Z80
    }
    fn registers(&mut self) -> &mut dyn RegisterOps {
        &mut self.registers
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}