use std::collections::HashMap;

pub trait IODevice: Send + Sync {
    fn ports(&self) -> Vec<u8>;
    fn read(&self, port: u8) -> Result<u8, &'static str>;
    fn write(&mut self, pin: u8, data: u8) -> Result<(), &'static str>;
    fn step(&mut self) {}
    /// Whether the device is holding INT active, asking for a maskable interrupt.
    /// For devices that drive INT themselves, like a Z80 CTC.
    fn int_pending(&self) -> bool {
        false
    }
    /// The CPU accepted the device's interrupt. Return the byte the device puts on
    /// the data bus (the instruction to run in IM 0, the low byte of the vector
    /// address in IM 2, ignored in IM 1), and normally stop holding INT.
    fn int_ack(&mut self) -> u8 {
        0xFF
    }
}

pub struct IORegister {
    pub registers: HashMap<u8, u8>,
}

impl IORegister {
    pub fn new(pins: Vec<u8>) -> IORegister {
        let mut registers = HashMap::new();
        for pin in pins {
            registers.insert(pin, 0);
        }
        IORegister { registers }
    }
}

impl IODevice for IORegister {
    fn ports(&self) -> Vec<u8> {
        self.registers.keys().copied().collect()
    }
    fn read(&self, port: u8) -> Result<u8, &'static str> {
        self.registers
            .get(&port)
            .copied()
            .ok_or("Attempting to read port not mapped to this device")
    }
    fn write(&mut self, port: u8, data: u8) -> Result<(), &'static str> {
        *self.registers.get_mut(&port).unwrap() = data;
        Ok(())
    }

    fn step(&mut self) {}
}

impl Default for IORegister {
    fn default() -> IORegister {
        let mut registers = HashMap::new();
        for pin in 0x00..0x100 {
            registers.insert(pin as u8, 0);
        }
        IORegister { registers }
    }
}
