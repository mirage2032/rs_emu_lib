//! What the Z80 does in place of an instruction from memory: accept an interrupt,
//! or idle while halted. `step()` returns each as a pseudo-instruction, so cycle
//! counts, callbacks and breakpoints see it like any other step.

use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{
    push_16, BaseInstruction, ExecutableInstruction, InstructionCommon, InstructionParser,
};
use crate::cpu::z80::parser::Z80_PARSER;
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};

#[derive(Debug, Clone, Copy)]
enum Source {
    Nmi,
    Int { mode: u8, data_bus: u8 },
}

/// The CPU accepting an interrupt: it pushes PC and jumps to the handler, in 11
/// T-states for NMI, 13 for IM 0 (RST) and IM 1, and 19 for IM 2. Displays as, for
/// example, `INT IM1 -> 0x0038`.
#[derive(Debug)]
pub struct InterruptAck {
    common: InstructionCommon,
    source: Source,
    /// IM 0: the instruction the device put on the data bus, unless it was an RST.
    bus_instruction: Option<Box<dyn ExecutableInstruction<Z80>>>,
    /// IM 2: the vector table entry the handler's address was read from.
    table: u16,
    /// Where execution continues once the interrupt is accepted.
    target: u16,
}

impl InterruptAck {
    pub(crate) fn nmi() -> InterruptAck {
        InterruptAck::new(Source::Nmi, 11)
    }

    pub(crate) fn int(mode: u8, data_bus: u8) -> InterruptAck {
        let cycles = if mode == 2 { 19 } else { 13 };
        InterruptAck::new(Source::Int { mode, data_bus }, cycles)
    }

    fn new(source: Source, cycles: u16) -> InterruptAck {
        InterruptAck {
            common: InstructionCommon::new(0, cycles, false),
            source,
            bus_instruction: None,
            table: 0,
            target: 0,
        }
    }
}

/// Decodes the instruction an interrupting device put on the data bus in IM 0. The
/// bus supplies a single byte, so only one-byte instructions fit.
fn bus_instruction(data_bus: u8) -> Result<Box<dyn ExecutableInstruction<Z80>>, String> {
    let not_one_byte =
        || format!("IM 0: 0x{data_bus:02X} on the data bus is not a one-byte instruction");
    let instruction = Z80_PARSER
        .ins_from_machinecode(&vec![data_bus], 0)
        .map_err(|_| not_one_byte())?;
    if instruction.common().length != 1 {
        return Err(not_one_byte());
    }
    Ok(instruction)
}

impl Display for InterruptAck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.source, &self.bus_instruction) {
            (Source::Nmi, _) => write!(f, "NMI")?,
            (Source::Int { mode: 0, .. }, Some(instruction)) => write!(f, "INT IM0 {instruction}")?,
            (Source::Int { mode: 0, data_bus }, None) => {
                write!(f, "INT IM0 RST 0x{:02X}", data_bus & 0x38)?
            }
            (Source::Int { mode: 2, .. }, _) => write!(f, "INT IM2 (0x{:04X})", self.table)?,
            (Source::Int { mode, .. }, _) => write!(f, "INT IM{mode}")?,
        }
        write!(f, " -> 0x{:04X}", self.target)
    }
}

impl BaseInstruction for InterruptAck {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![] // nothing is fetched from memory
    }
}

impl ExecutableInstruction<Z80> for InterruptAck {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, io: &mut IO) -> Result<(), String> {
        match self.source {
            Source::Nmi => {
                // IFF2 keeps IFF1's state for RETN. It isn't written, so a nested NMI
                // doesn't lose it.
                cpu.interrupts.nmi_pending = false;
                cpu.interrupts.iff1 = false;
                cpu.halted = false;
                push_16!(cpu.registers.pc, memory, cpu.registers.sp);
                cpu.registers.pc = 0x0066;
            }
            Source::Int { mode, data_bus } => {
                if mode > 2 {
                    return Err(format!("Invalid interrupt mode: {mode}"));
                }
                // Decode IM 0's instruction before IFF, PC or SP change, so a byte that
                // isn't a one-byte instruction is reported without touching them.
                let rst = data_bus & 0xC7 == 0xC7;
                if mode == 0 && !rst {
                    self.bus_instruction = Some(bus_instruction(data_bus)?);
                }
                cpu.interrupts.iff1 = false;
                cpu.interrupts.iff2 = false;
                cpu.halted = false;
                match (mode, &mut self.bus_instruction) {
                    (0, Some(instruction)) => {
                        // It runs without moving PC, 2 T-states slower than from memory.
                        instruction.execute(memory, cpu, io)?;
                        self.common.cycles = instruction.common().cycles + 2;
                    }
                    (0, None) => {
                        // An RST from the bus pushes PC unchanged, so the interrupted
                        // instruction runs on return.
                        push_16!(cpu.registers.pc, memory, cpu.registers.sp);
                        cpu.registers.pc = (data_bus & 0x38) as u16;
                    }
                    (1, _) => {
                        push_16!(cpu.registers.pc, memory, cpu.registers.sp);
                        cpu.registers.pc = 0x0038;
                    }
                    _ => {
                        self.table = u16::from_le_bytes([data_bus, cpu.registers.i]);
                        push_16!(cpu.registers.pc, memory, cpu.registers.sp);
                        cpu.registers.pc = memory.read_16(self.table)?;
                    }
                }
            }
        }
        self.target = cpu.registers.pc;
        Ok(())
    }
}

/// One idle step of a halted CPU, returned by `step()` until an interrupt is
/// accepted: 4 T-states, like a NOP, without moving PC. PC already points past the
/// HALT, where the CPU resumes.
#[derive(Debug)]
pub struct Halted {
    common: InstructionCommon,
}

impl Halted {
    pub(crate) fn new() -> Halted {
        Halted {
            common: InstructionCommon::new(0, 4, false),
        }
    }
}

impl Display for Halted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HALT (idle)")
    }
}

impl BaseInstruction for Halted {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![] // nothing is fetched from memory
    }
}

impl ExecutableInstruction<Z80> for Halted {
    fn execute(&mut self, _memory: &mut Memory, _cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        Ok(())
    }
}
