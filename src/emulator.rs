use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::Cpu;
use crate::io::IO;
use crate::memory::{Memory, MemoryDevice};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum StopReason {
    Breakpoint,
    /// The CPU is halted with maskable interrupts disabled, so only an NMI can wake
    /// it. A halted CPU that an INT could wake keeps running (idling) instead.
    Halt,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmuState {
    pub cpu: Vec<u8>,
    pub memory: Vec<u8>,
    pub breakpoints: Vec<u16>,
}

pub struct Emulator<T: Cpu> {
    pub memory: Memory,
    pub cpu: T,
    pub breakpoints: Vec<u16>,
    pub io: IO,
    pub cycles: usize,
    pub instructions: usize,
}

impl<T: Cpu+'static> Default for Emulator<T> {
    fn default() -> Emulator<T> {
        Emulator {
            memory: Memory::default(),
            cpu: T::default(),
            breakpoints: Vec::new(),
            io: IO::default(),
            cycles: 0,
            instructions: 0,
        }
    }
}

impl<T: Cpu +'static> Emulator<T> {
    pub fn new_w_mem(memory: Memory) -> Emulator<T> {
        Emulator {
            memory,
            cpu: T::default(),
            breakpoints: Vec::new(),
            io: IO::default(),
            cycles: 0,
            instructions: 0,
        }
    }
    /// Runs one step: an instruction, the acceptance of an interrupt, or one idle
    /// cycle of a halted CPU.
    pub fn step(&mut self) -> Result<Box<dyn ExecutableInstruction<T>>, String> {
        self.memory.clear_changes();
        let instruction = self.cpu.step(&mut self.memory, &mut self.io);
        self.io.step();
        if let Ok(instruction) = &instruction {
            self.cycles += instruction.common().cycles as usize;
            self.instructions += 1;
        }
        instruction
    }

    pub fn run_ticks<CB: Fn(&mut Self, &dyn ExecutableInstruction<T>)>(
        &mut self,
        ticks: f64,
        callback: &Option<CB>,
    ) -> Result<f64, StopReason> {
        let mut current_ticks = 0.0;
        while current_ticks < ticks {
            if self.cpu.halted() && self.cpu.deadlocked() {
                return Err(StopReason::Halt);
            }
            let instruction = self.step().map_err(|e| StopReason::Error(e))?;
            current_ticks += instruction.common().cycles as f64;
            if let Some(callback) = &callback {
                callback(self, &*instruction);
            }
            // A halted CPU stays at the same PC; a breakpoint there fires once the
            // CPU gets back to it by running, not on every idle cycle.
            if !self.cpu.halted() && self.breakpoints.contains(&self.cpu.pc()) {
                return Err(StopReason::Breakpoint);
            }
        }
        Ok(current_ticks)
    }

    pub fn run_with_callback<CB: Fn(&mut Self, &dyn ExecutableInstruction<T>)>(
        &mut self,
        frequency: f32,
        callback: Option<CB>,
        ticks_per_chunk: f64,
    ) -> StopReason {
        let tick_duration = Duration::from_secs_f64(1.0 / frequency as f64);

        loop {
            let time_before = SystemTime::now();
            let res = self.run_ticks(ticks_per_chunk, &callback);
            let ticks = match res {
                Ok(ticks) => ticks,
                Err(e) => {
                    return e;
                }
            };
            let exec_duration = tick_duration * ticks as u32;
            let expected_finish = time_before + exec_duration;
            let time_after = SystemTime::now();
            if let Ok(difference) = expected_finish.duration_since(time_after) {
                // println!("Sleeping for {:?}", difference);
                std::thread::sleep(difference)
            } else {
                println!("Warning: Emulator is unable to keep up required frequency of {}Hz", frequency);
            }
        }
    }

    pub fn run(&mut self, frequency: f32, ticks_per_chunk: f64) -> StopReason {
        self.run_with_callback(
            frequency,
            None::<fn(&mut Self, &dyn ExecutableInstruction<T>)>,
            ticks_per_chunk,
        )
    }

    pub fn save(&self) -> Result<Vec<u8>, String> {
        let memory = self.memory.save().map_err(|e| format!("{:?}", e))?;
        let cpu = bincode::serialize(&self.cpu).map_err(|e| format!("{:?}", e))?;
        let state = EmuState {
            cpu,
            memory,
            breakpoints: self.breakpoints.clone(),
        };
        bincode::serialize(&state).map_err(|e| format!("{:?}", e))
    }
    pub fn load(&mut self, data: Vec<u8>, clear_mem: bool, force: bool) -> Result<(), String> {
        let state = bincode::deserialize::<EmuState>(&data).map_err(|e| format!("{:?}", e))?;
        self.cpu = bincode::deserialize::<T>(&state.cpu).map_err(|e| format!("{:?}", e))?;
        self.memory
            .load(&state.memory, force)
            .map_err(|e| format!("{:?}", e))?;
        if clear_mem {
            for idx in state.memory.len()..self.memory.size() {
                self.memory
                    .write_8(idx as u16, 0)
                    .map_err(|e| format!("{:?}", e))?;
            }
        }
        self.breakpoints = state.breakpoints;
        Ok(())
    }
    pub fn reset_counters(&mut self) {
        self.cycles=0;
        self.instructions=0;
    }

    /// Requests a non-maskable interrupt, taken before the next instruction.
    pub fn request_nmi(&mut self) {
        self.cpu.request_nmi();
    }

    /// Requests a maskable interrupt (INT), held until the CPU accepts it: a request
    /// made while interrupts are disabled waits for EI, and repeated requests merge
    /// into one. `data_bus` is the byte the interrupting device supplies: the
    /// instruction to run in IM 0 (0xFF is RST 38h), the low byte of the vector
    /// address in IM 2, and ignored in IM 1. For an interrupt that is only active
    /// for a while (a Spectrum's lasts 32 T-states), use `set_int_line`.
    pub fn request_int(&mut self, data_bus: u8) {
        self.cpu.request_int(data_bus);
    }

    /// Holds INT active with `data_bus` (`asserted`), or releases it. While held, the
    /// interrupt is taken again each time interrupts are enabled.
    pub fn set_int_line(&mut self, asserted: bool, data_bus: u8) {
        self.cpu.set_int_line(asserted, data_bus);
    }
}
