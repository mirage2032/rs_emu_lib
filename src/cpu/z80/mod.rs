use crate::cpu::instruction::{ExecutableInstruction, InstructionParser};
use crate::cpu::registers::{AllMutRegisters, AllRegisters, GPByteRegisters};
use crate::cpu::Cpu;
use crate::io::IO;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::super::memory::Memory;
use interrupt::{Halted, InterruptAck};

mod alu;
pub mod instructions;
pub mod interrupt;
pub mod parser;

#[cfg(test)]
mod interrupt_tests;
#[cfg(test)]
mod roundtrip;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Z80Registers {
    pub gp: GPByteRegisters,
    pub gp_alt: GPByteRegisters,
    pub ix: u16,
    pub iy: u16,
    pub i: u8,
    pub r: u8,
    pub sp: u16,
    pub pc: u16,
}
impl Z80Registers {
    /// Exchanges BC, DE and HL with their alternates, as EXX does. AF isn't
    /// exchanged: EX AF,AF' does that.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.gp.bc, &mut self.gp_alt.bc);
        std::mem::swap(&mut self.gp.de, &mut self.gp_alt.de);
        std::mem::swap(&mut self.gp.hl, &mut self.gp_alt.hl);
    }
    /// Counts R up, as each opcode fetch does. Only the low 7 bits count; bit 7
    /// keeps what LD R,A put there.
    pub fn inc_r(&mut self) {
        self.r = (self.r & 0x80) | (self.r.wrapping_add(1) & 0x7F);
    }
}
impl Default for Z80Registers {
    fn default() -> Self {
        Z80Registers {
            gp: GPByteRegisters::default(),
            gp_alt: GPByteRegisters::default(),
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            sp: 0xFFFF,
            pc: 0,
        }
    }
}
/// IX or IY. Each instruction that uses one has a twin that uses the other and
/// differs only in its prefix byte, so the pair is one type, generic over this
/// trait: `ADD_A_PIDXD<IX>`, also named `ADD_A_PIXD`, is `ADD A,(IX+d)`.
pub trait IndexRegister: std::fmt::Debug + Send + Sync + 'static {
    /// The prefix byte: DD for IX, FD for IY.
    const PREFIX: u8;
    /// The register's name in assembly.
    const NAME: &'static str;
    fn get(registers: &Z80Registers) -> u16;
    fn get_mut(registers: &mut Z80Registers) -> &mut u16;
}

#[derive(Debug)]
pub struct IX;

#[derive(Debug)]
pub struct IY;

impl IndexRegister for IX {
    const PREFIX: u8 = 0xDD;
    const NAME: &'static str = "IX";
    fn get(registers: &Z80Registers) -> u16 {
        registers.ix
    }
    fn get_mut(registers: &mut Z80Registers) -> &mut u16 {
        &mut registers.ix
    }
}

impl IndexRegister for IY {
    const PREFIX: u8 = 0xFD;
    const NAME: &'static str = "IY";
    fn get(registers: &Z80Registers) -> u16 {
        registers.iy
    }
    fn get_mut(registers: &mut Z80Registers) -> &mut u16 {
        &mut registers.iy
    }
}

/// The Z80's interrupt state: the interrupt flip-flops, the interrupt mode, and the
/// requests waiting to be accepted. It is part of the CPU, so save states carry it
/// and `Z80::default()` (a reset) clears it.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterruptState {
    /// IFF1: maskable interrupts (INT) are accepted only while it is set.
    pub iff1: bool,
    /// IFF2: keeps IFF1's state while an NMI is serviced. RETN and RETI copy it back
    /// into IFF1, and LD A,I reads it into P/V.
    pub iff2: bool,
    /// The interrupt mode set by IM 0, IM 1 or IM 2.
    pub im: u8,
    /// Set by EI: INT isn't accepted until the instruction after EI has run.
    pub ei_delay: bool,
    /// An NMI is waiting to be accepted. NMI is edge-triggered, so a request latches.
    pub nmi_pending: bool,
    /// INT held active (level-triggered) by `set_int_line`, with the byte the
    /// interrupting device puts on the data bus.
    pub int_line: Option<u8>,
    /// INT requested by `request_int`, with its data-bus byte. Cleared when accepted.
    pub int_request: Option<u8>,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Z80 {
    pub registers: Z80Registers,
    pub interrupts: InterruptState,
    halted: bool,
}

impl Z80 {
    /// The data-bus byte of the maskable interrupt to accept before the next
    /// instruction, if INT is active and enabled. A `request_int` comes first, then
    /// the line held by `set_int_line`, then the first device holding INT, which is
    /// acknowledged.
    fn int_to_accept(&mut self, after_ei: bool, io: &mut IO) -> Option<u8> {
        if !self.interrupts.iff1 || after_ei {
            return None;
        }
        self.interrupts
            .int_request
            .take()
            .or(self.interrupts.int_line)
            .or_else(|| io.int_ack())
    }
}

impl Cpu for Z80 {
    fn step(
        &mut self,
        memory: &mut Memory,
        io: &mut IO,
    ) -> Result<Box<dyn ExecutableInstruction<Self>>, String> {
        // Interrupts are checked before each instruction, NMI first. EI holds INT off
        // for one instruction, but never NMI.
        let after_ei = std::mem::take(&mut self.interrupts.ei_delay);
        let mut instruction: Box<dyn ExecutableInstruction<Z80>> = if self.interrupts.nmi_pending {
            Box::new(InterruptAck::nmi())
        } else if let Some(data_bus) = self.int_to_accept(after_ei, io) {
            Box::new(InterruptAck::int(self.interrupts.im, data_bus))
        } else if self.halted {
            Box::new(Halted::new())
        } else {
            parser::Z80_PARSER
                .ins_from_machinecode(memory, self.registers.pc)
                .map_err(|e| e.to_string())?
        };
        // Each opcode fetch counts R up: two for a prefixed instruction (the CB, DD,
        // ED or FD and the opcode after it), one for anything else, including
        // accepting an interrupt and an idle HALT step. They count before the
        // instruction runs, so LD A,R reads R with both of its fetches counted.
        let fetches = match instruction.to_bytes().first() {
            Some(0xCB | 0xDD | 0xED | 0xFD) => 2,
            _ => 1,
        };
        for _ in 0..fetches {
            self.registers.inc_r();
        }
        instruction.execute(memory, self, io)?;
        let common = instruction.common();
        if common.increment_pc {
            let inst_length = common.length;
            let new_pc = self.registers.pc.wrapping_add(inst_length);
            self.registers.pc = new_pc;
        }
        Ok(instruction)
    }
    fn parser(&self) -> &dyn InstructionParser<Z80> {
        &parser::Z80_PARSER
    }

    fn registers(&self) -> AllRegisters<'_> {
        let mut other8bit = HashMap::new();
        let mut other16bit = HashMap::new();
        other16bit.insert("ix", &self.registers.ix);
        other16bit.insert("iy", &self.registers.iy);
        other8bit.insert("i", &self.registers.i);
        other8bit.insert("r", &self.registers.r);
        AllRegisters {
            gp: vec![&self.registers.gp, &self.registers.gp_alt],
            other8bit,
            other16bit,
            sp: &self.registers.sp,
            pc: &self.registers.pc,
        }
    }
    fn registers_mut(&mut self) -> AllMutRegisters<'_> {
        let mut other8bit = HashMap::new();
        let mut other16bit = HashMap::new();
        other16bit.insert("ix", &mut self.registers.ix);
        other16bit.insert("iy", &mut self.registers.iy);
        other8bit.insert("i", &mut self.registers.i);
        other8bit.insert("r", &mut self.registers.r);
        AllMutRegisters {
            gp: vec![&mut self.registers.gp, &mut self.registers.gp_alt],
            other8bit,
            other16bit,
            sp: &mut self.registers.sp,
            pc: &mut self.registers.pc,
        }
    }

    fn pc(&self) -> u16 {
        self.registers.pc
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn set_halted(&mut self, halted: bool) {
        self.halted = halted;
    }
    fn request_nmi(&mut self) {
        self.interrupts.nmi_pending = true;
    }
    fn request_int(&mut self, data_bus: u8) {
        self.interrupts.int_request = Some(data_bus);
    }
    fn set_int_line(&mut self, asserted: bool, data_bus: u8) {
        self.interrupts.int_line = asserted.then_some(data_bus);
    }
    fn deadlocked(&self) -> bool {
        self.halted && !self.interrupts.iff1 && !self.interrupts.nmi_pending
    }
}
