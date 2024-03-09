use std::time::{Duration, SystemTime};

use crate::emu_lib::cpu::{BaseInstruction, Cpu, CPUType};
use crate::emu_lib::cpu::i8080::I8080;
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::io::IO;
use crate::emu_lib::memory::Memory;

pub enum StopReason {
    Breakpoint,
    Halt,
    Error(String),
}

pub struct Emulator {
    pub memory: Memory,
    pub cpu: Box<dyn Cpu>,
    pub breakpoints: Vec<u16>,
    pub io: IO,
}

impl Emulator {
    pub fn new(cpu_type: CPUType) -> Emulator {
        let cpu: Box<dyn Cpu> = match cpu_type {
            CPUType::Z80 => Box::new(Z80::new()),
            CPUType::I8080 => Box::new(I8080::new())
        };
        Emulator {
            memory: Memory::default(),
            cpu,
            breakpoints: Vec::new(),
            io: IO::default(),
        }
    }
    pub fn new_w_mem(cpu_type: CPUType, memory: Memory) -> Emulator {
        let cpu: Box<dyn Cpu> = match cpu_type {
            CPUType::Z80 => Box::new(Z80::new()),
            CPUType::I8080 => Box::new(I8080::new())
        };
        Emulator {
            memory,
            cpu,
            breakpoints: Vec::new(),
            io: IO::default(),
        }
    }
    pub fn step(&mut self) -> Result<Box<dyn BaseInstruction>, String> {
        if self.cpu.halted() {
            return Err("CPU is halted".to_string());
        }
        self.cpu.step(&mut self.memory, &mut self.io)
    }

    pub fn run_w_cb<T: Fn(&mut Self, &dyn BaseInstruction)>(&mut self, frequency: f32, callback: Option<T>) -> StopReason
    {
        let tick_duration = Duration::from_secs_f32(1.0 / frequency);
        let mut last_tick_time = SystemTime::now();

        loop {
            let current_time = SystemTime::now();
            let elapsed_time = current_time.duration_since(last_tick_time).unwrap();
            let instruction = match self.step() {
                Ok(instructions) => { instructions }
                Err(e) => return StopReason::Error(e),
            };
            if let Some(cb) = &callback {
                cb(self, instruction.as_ref());
            }
            let instruction_time = tick_duration * instruction.common().get_cycles() as u32;

            if self.cpu.halted() {
                return StopReason::Halt;
            }

            if self.breakpoints.contains(self.cpu.registers().pc()) {
                return StopReason::Breakpoint;
            }

            // Calculate remaining time to sleep
            let remaining_time = if elapsed_time < tick_duration {
                tick_duration - elapsed_time
            } else {
                Duration::from_secs(0)
            };
            last_tick_time = SystemTime::now();
            // Sleep only if there's remaining time and execution time is less than tick duration
            if instruction_time < tick_duration {
                std::thread::sleep(remaining_time);
            }
        }
    }

    pub fn run(&mut self, frequency: f32) -> StopReason {
        self.run_w_cb(frequency, None::<fn(&mut Self, &dyn BaseInstruction)>)
    }
    pub fn set_cpu_type(&mut self, cpu_type: CPUType) {
        if self.cpu.type_of() == cpu_type {
            return;
        }
        self.cpu = match cpu_type {
            CPUType::Z80 =>
                Box::new(Z80::new()),
            CPUType::I8080 =>
                Box::new(I8080::new())
        };
    }
}