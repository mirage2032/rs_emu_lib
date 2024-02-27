use std::time::{Duration, SystemTime};

use super::cpu::{Cpu, CPUType, i8080::I8080, z80::Z80};
use super::memory::Memory;

enum StopReason {
    Breakpoint,
    Halt,
    Error(String),
}

struct Emulator {
    pub memory: Memory,
    pub cpu: Box<dyn Cpu>,
    pub breakpoints: Vec<u16>,
}

impl Emulator {
    fn new(cpu_type: CPUType) -> Emulator {
        let cpu: Box<dyn Cpu> = match cpu_type {
            CPUType::Z80 => Box::new(Z80::new()),
            CPUType::I8080 => Box::new(I8080::new())
        };
        Emulator {
            memory: Memory::new(),
            cpu,
            breakpoints: Vec::new(),
        }
    }

    fn step(&mut self) -> Result<u16, String> {
        self.cpu.step(&mut self.memory)
    }

    fn run(&mut self, frequency: f32) -> StopReason {
        let tick_duration = Duration::from_secs_f32(1.0 / frequency);
        let mut last_tick_time = SystemTime::now();

        loop {
            let current_time = SystemTime::now();
            let elapsed_time = current_time.duration_since(last_tick_time).unwrap();
            let cycles = match self.step() {
                Ok(cycles) => { cycles }
                Err(e) => return StopReason::Error(e),
            };
            let instruction_time = tick_duration * cycles as u32;

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

    fn set_cpu_type(&mut self, cpu_type: CPUType) {
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