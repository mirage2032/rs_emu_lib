use crate::cpu::registers::AllRegisters;
use crate::emu_lib::cpu::instruction::{BaseInstruction, InstructionParser};
use crate::emu_lib::cpu::{CPUType, Cpu};
use crate::emu_lib::io::IO;

use super::super::memory::Memory;

pub struct I8080 {
    halted: bool,
}

impl Default for I8080 {
    fn default() -> I8080 {
        I8080 { halted: false }
    }
}

impl Cpu for I8080 {
    fn step(&mut self, _: &mut Memory, _: &mut IO) -> Result<Box<dyn BaseInstruction>, String> {
        unimplemented!()
    }
    fn type_of(&self) -> CPUType {
        CPUType::I8080
    }
    fn parser(&self) -> &dyn InstructionParser {
        unimplemented!()
    }

    fn registers(&self) -> &AllRegisters {
        unimplemented!()
    }
    fn registers_mut(&mut self) -> &mut AllRegisters {
        unimplemented!()
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
}
