use super::cpu::{Cpu, CPUType, i8080::I8080, z80::Z80};
use super::memory::Memory;

struct Emulator<> {
    memory: Memory,
    cpu_type: CPUType,
    cpu: Box<dyn Cpu>,
}

impl Emulator {
    fn new(cpu_type: CPUType) -> Emulator {
        let cpu: Box<dyn Cpu> = match cpu_type {
            CPUType::Z80 => Box::new(Z80::new()),
            CPUType::I8080 => Box::new(I8080::new())
        };
        Emulator {
            memory: Memory::new(),
            cpu_type,
            cpu,
        }
    }

    fn step(&mut self) -> u16 {
        self.cpu.step(&mut self.memory)
    }

    fn set_cpu_type(&mut self, cpu_type: CPUType) {
        if self.cpu_type == cpu_type {
            return;
        }
        self.cpu_type = cpu_type;
        self.cpu = match self.cpu_type {
            CPUType::Z80 =>
                Box::new(Z80::new()),
            CPUType::I8080 =>
                Box::new(I8080::new())
        };
    }

    fn get_cpu_type(&self) -> &CPUType {
        &self.cpu_type
    }
}