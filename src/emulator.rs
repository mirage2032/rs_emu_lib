use std::time::{Duration, SystemTime};

use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::Cpu;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub enum StopReason {
    Breakpoint,
    Halt,
    Error(String),
}

pub struct Emulator<T: Cpu> {
    pub memory: Memory,
    pub cpu: T,
    pub breakpoints: Vec<u16>,
    pub io: IO,
    pub cycles: usize,
}

impl<T: Cpu> Emulator<T> {
    pub fn new() -> Emulator<T> {
        Emulator {
            memory: Memory::default(),
            cpu: T::default(),
            breakpoints: Vec::new(),
            io: IO::default(),
            cycles: 0,
        }
    }
    pub fn new_w_mem(memory: Memory) -> Emulator<T> {
        Emulator {
            memory,
            cpu: T::default(),
            breakpoints: Vec::new(),
            io: IO::default(),
            cycles: 0,
        }
    }
    pub fn step(&mut self) -> Result<Box<dyn ExecutableInstruction<T>>, String> {
        if self.cpu.halted() {
            return Err("CPU is halted".to_string());
        }
        self.memory.clear_changes();
        let instruction = self.cpu.step(&mut self.memory, &mut self.io);
        if let Ok(instruction) = &instruction {
            self.cycles += instruction.common().cycles as usize;
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
            let instruction = self.step().map_err(|e| StopReason::Error(e))?;
            current_ticks += instruction.common().cycles as f64;
            if let Some(callback) = &callback {
                callback(self, &*instruction);
            }
            if self.cpu.halted() {
                return Err(StopReason::Halt);
            }
            if self.breakpoints.contains(&self.cpu.pc()) {
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
                println!("Warning: Emulator is running too slow");
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
}
