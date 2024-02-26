use super::cpu::{Cpu, CPUType, i8080::I8080, z80::Z80};
use super::memory::Memory;

struct Emulator {
    pub memory: Memory,
    pub cpu: Box<dyn Cpu>,
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
        }
    }

    fn step(&mut self) -> u16 {
        self.cpu.step(&mut self.memory)
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