use registers::Registers;

use crate::emu_lib::cpu::{BaseInstruction, Cpu, CPUType, InstructionParser, RegisterOps};
use crate::emu_lib::cpu::ExecutableInstruction;
use crate::emu_lib::io::{InterruptType, IO};
use crate::push_16;

use super::super::memory::{Memory, MemoryDevice};

mod registers;
pub mod instructions;
mod parser;

pub struct Z80 {
    registers: Registers,
    parser: parser::Z80Parser,
    halted: bool,
}

impl Default for Z80 {
    fn default() -> Self {
        Z80 {
            registers: Registers::default(),
            parser: parser::Z80Parser::new(),
            halted: false,
        }
    }
}

impl Z80 {
    fn handle_interrupt(&mut self, memory: &mut Memory, io: &mut IO) -> Result<Option<Box<dyn ExecutableInstruction<Z80>>>, String> {
        match io.get_interrupt() {
            Some((int_vector, id)) => {
                let ret_instr: Option<Box<dyn ExecutableInstruction<Z80>>> = match int_vector {
                    InterruptType::NMI => {
                        self.registers.pc = 0x66;
                        None
                    }
                    InterruptType::IM0(instruction) => {
                        let instruction = parser::Z80Parser::from_memdev(&vec![instruction], 0)?;
                        instruction.execute(memory, self, io)?;
                        Some(instruction)
                    }
                    remaining => {
                        push_16!(self.registers.pc,memory,self.registers.sp);
                        self.registers.swap(); // TODO: Check: Should be done???
                        match remaining {
                            InterruptType::IM1 => {
                                self.registers.pc = 0x38;
                            }
                            InterruptType::IM2(int_vector) => {
                                self.registers.pc = u16::from_le_bytes([int_vector, self.registers.i]);
                            }
                            _ => unreachable!("IM0/NMI should have been handled")
                        }
                        None
                    }
                };
                io.ack_int(id)?;
                Ok(ret_instr)
            }
            None => { Ok(None) }
        }
    }
}

impl Cpu for Z80 {
    fn step(&mut self, memory: &mut Memory, io: &mut IO) -> Result<Box<(dyn BaseInstruction)>, String> {
        let res = self.handle_interrupt(memory, io)?; // If IM1 interrupt it will be returned and executed
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match res {
            Some(instruction) => instruction,
            None => parser::Z80Parser::from_memdev(memory, self.registers.pc)?
        };
        instruction.execute(memory, self, io)?;
        Ok(instruction)
    }
    fn parser(&self) -> &dyn InstructionParser {
        &self.parser
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