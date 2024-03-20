use std::collections::HashMap;

use super::InterruptType;

pub trait IODevice: Send {
    fn ports(&self) -> Vec<u8>;
    fn read(&self, pin: u8) -> Result<u8, &'static str>;
    fn write(&mut self, pin: u8, data: u8) -> Result<(), &'static str>;
    fn step(&mut self);
    fn will_interrupt(&self) -> Option<InterruptType>;
    fn ack_int(&mut self) -> Result<(), &'static str>;
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
    fn ports(&self) -> Vec<u8> {
        self.registers.keys().copied().collect()
    }
    fn read(&self, pin: u8) -> Result<u8, &'static str> {
        self.registers.get(&pin).copied().ok_or("Attempting to read port not mapped to this device")
    }
    fn write(&mut self, pin: u8, data: u8) -> Result<(), &'static str> {
        *self.registers.get_mut(&pin).unwrap() = data;
        Ok(())
    }

    fn step(&mut self) {}
    fn will_interrupt(&self) -> Option<InterruptType> {
        None
    }
    fn ack_int(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}

impl Default for IORegister {
    fn default() -> IORegister {
        let mut registers = HashMap::new();
        for pin in 0x00..0x100 {
            registers.insert(pin as u8, 0);
        }
        IORegister {
            registers
        }
    }
}