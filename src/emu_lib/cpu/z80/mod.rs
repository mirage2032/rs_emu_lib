use registers::Registers;

use crate::cpu::ExecutableInstruction;
use crate::emu_lib::cpu::{BaseInstruction, Cpu, CPUType, InstructionDecoder, InstructionEncoder, RegisterOps};
use crate::io::{IO, iodevice::InterruptType};

use super::super::memory::{Memory, WriteableMemory};

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

    fn handle_interrupt(&mut self, memory: &mut Memory, io: &mut IO) -> Result<Option<Box<dyn ExecutableInstruction<Z80>>>, String> {
        match io.get_interrupt() {
            Some((int_vector, id)) => {
                let mut ret_instr: Option<Box<dyn ExecutableInstruction<Z80>>> = None;
                match int_vector {
                    InterruptType::IM0(instruction) => {
                        let instruction = Self::decode(&vec![instruction], 0)?;
                        instruction.execute(memory, self, io)?;
                        ret_instr = Some(instruction);
                    }
                    _ => {
                        memory.write_16(self.registers.sp - 2, self.registers.pc + 1).or_else(|_| Err("Error pushing SP to stack durring interrupt".to_string()))?;
                        self.registers.sp -= 2;
                        match int_vector {
                            InterruptType::IM1 => {
                                self.registers.pc = 0x38;
                            }
                            InterruptType::IM2(int_vector) => {
                                self.registers.pc = u16::from_le_bytes([int_vector, self.registers.i]);
                            }
                            _ => unreachable!("IM0 should have been handled")
                        }
                    }
                }
                io.ack_int(id)?;
                return Ok(ret_instr);
            }
            None => { Ok(None) }
        }
    }
}

impl Cpu for Z80 {
    fn step(&mut self, memory: &mut Memory, io: &mut IO) -> Result<Box<(dyn BaseInstruction)>, String> {
        let res = self.handle_interrupt(memory, io)?; // If IM1 interrupt it will be returned and executed
        let instruction = match res {
            Some(instruction) => instruction,
            None => Self::decode(memory, self.registers.pc)?
        };
        instruction.execute(memory, self, io)?;
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