use std::time::{Duration, SystemTime};

use crate::emu_lib::cpu::i8080::I8080;
use crate::emu_lib::cpu::instruction::BaseInstruction;
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::cpu::{CPUType, Cpu};
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub enum StopReason {
    Breakpoint,
    Halt,
    Error(String),
}

pub struct Emulator<T:Cpu+Default> {
    pub memory: Memory,
    pub cpu: T,
    pub breakpoints: Vec<u16>,
    pub io: IO,
    pub cycles: usize,
}

impl<T:Cpu+Default> Emulator<T> {
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
    pub fn step(&mut self) -> Result<Box<dyn BaseInstruction>, String> {
        if self.cpu.halted() {
            return Err("CPU is halted".to_string());
        }
        self.memory.clear_changes();
        let instruction = self.cpu.step(&mut self.memory, &mut self.io);
        if let Ok(instruction) = &instruction {
            self.cycles += instruction.common().get_cycles() as usize;
        }
        instruction
    }

    pub fn run_w_cb<CB: Fn(&mut Self, &dyn BaseInstruction)>(
        &mut self,
        frequency: f32,
        callback: Option<CB>,
        ticks_per_chunk: usize,
    ) -> StopReason {
        let tick_duration = Duration::from_secs_f32(1.0 / frequency);

        let mut current_ticks = 0;
        loop {
            let time_before = SystemTime::now();
            while current_ticks < ticks_per_chunk {
                let instruction = match self.step() {
                    Ok(instructions) => instructions,
                    Err(e) => return StopReason::Error(e),
                };
                current_ticks += instruction.common().get_cycles() as usize;
                if let Some(cb) = &callback {
                    cb(self, instruction.as_ref());
                }

                if self.cpu.halted() {
                    return StopReason::Halt;
                }

                if self.breakpoints.contains(&self.cpu.registers().pc) {
                    return StopReason::Breakpoint;
                }
            }
            let exec_duration = tick_duration * current_ticks as u32;
            let expected_finish = time_before + exec_duration;
            let time_after = SystemTime::now();
            current_ticks = current_ticks % ticks_per_chunk;
            if let Ok(difference) = expected_finish.duration_since(time_after) {
                // println!("Sleeping for {:?}", difference);
                std::thread::sleep(difference)
            }
            else {
                // println!("Warning: Emulator is running too slow");
            }
        }
    }

    pub fn run(&mut self, frequency: f32,ticks_per_chunk:usize) -> StopReason {
        self.run_w_cb(frequency, None::<fn(&mut Self, &dyn BaseInstruction)>,ticks_per_chunk)
    }
}
