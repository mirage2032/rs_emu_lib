use instruction::{BaseInstruction, InstructionParser};
use serde::{Deserialize, Serialize};

use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::registers::{AllMutRegisters, AllRegisters};
use crate::io::IO;
use crate::memory::Memory;

pub mod i8080;
pub mod instruction;
pub mod registers;
pub mod z80;

pub trait Cpu: Send + Copy + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn step(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Box<dyn ExecutableInstruction<Self>>, String>;
    fn parser(&self) -> &dyn InstructionParser<Self>;
    fn registers(&self) -> AllRegisters<'_>;
    fn registers_mut(&mut self) -> AllMutRegisters<'_>;
    fn pc(&self) -> u16;
    fn halted(&self) -> bool;
    fn set_halted(&mut self, halted: bool);
    /// Latch a non-maskable interrupt. The CPU takes it before its next instruction.
    fn request_nmi(&mut self);
    /// Request a maskable interrupt (INT). The request waits until the CPU accepts it
    /// (when interrupts are enabled), and repeated requests merge into one.
    /// `data_bus` is the byte the interrupting device supplies: the instruction to
    /// run in IM 0 (usually an RST), the low byte of the vector address in IM 2, and
    /// ignored in IM 1.
    fn request_int(&mut self, data_bus: u8);
    /// Hold INT active with `data_bus` (level-triggered, so the interrupt is taken
    /// again each time interrupts are enabled), or release it.
    fn set_int_line(&mut self, asserted: bool, data_bus: u8);
    /// Halted with maskable interrupts disabled and no NMI waiting: only an NMI can
    /// wake the CPU.
    fn deadlocked(&self) -> bool;
}
