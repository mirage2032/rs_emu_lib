use std::collections::HashMap;

pub enum InterruptType { // disable itnerrupts when it happens
    IM0(u8), // instruction to exec, usually RST xx, no save of PC by default
    IM1,     // jump to 0x0038 after pushing PC to stack
    IM2(u8), // jump to I + this after pushing PC to stack
}

pub trait IODevice {
    fn pins(&self) -> Vec<u8>;
    fn read(&self, pin: u8) -> Result<u8, &str>;
    fn write(&mut self, pin: u8, data: u8) -> Result<(), &str>;
    fn step(&mut self);
    fn will_interrupt(&self) -> Option<InterruptType>;
    fn ack_int(&mut self) -> Result<(), &str>;
}

pub struct IORegister {
    pub registers: HashMap<u8, u8>,
}

impl IORegister {
    fn new(pins: Vec<u8>) -> IORegister {
        let mut registers = HashMap::new();
        for pin in pins {
            registers.insert(pin, 0);
        }
        IORegister {
            registers
        }
    }
}

impl IODevice for IORegister {
    fn pins(&self) -> Vec<u8> {
        self.registers.keys().map(|k| *k).collect()
    }
    fn read(&self, pin: u8) -> Result<u8, &str> {
        self.registers.get(&pin).map(|v| *v).ok_or("Attempting to read port not mapped to this device")
    }
    fn write(&mut self, pin: u8, data: u8) -> Result<(), &str>{
        *self.registers.get_mut(&pin).unwrap() = data;
        Ok(())
    }

    fn step(&mut self) {}
    fn will_interrupt(&self) -> Option<InterruptType> {
        None
    }
    fn ack_int(&mut self) -> Result<(), &str> {
        Ok(())
    }
}

impl Default for IORegister {
    fn default() -> IORegister {
        let mut registers = HashMap::new();
        for pin in 0x00..0xFF {
            registers.insert(pin, 0);
        }
        IORegister {
            registers: HashMap::new()
        }
    }
}