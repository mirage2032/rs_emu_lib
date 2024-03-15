use std::collections::HashMap;

use iodevice::IODevice;
use iodevice::IORegister;

pub mod iodevice;

pub enum InterruptType {
    NMI,
    // non-maskable interrupt
    IM0(u8),
    // instruction to exec, usually RST xx, no save of PC by default
    IM1,
    // jump to 0x0038 after pushing PC to stack
    IM2(u8), // jump to val = I[msb] | u8[lsb]
}

pub struct IO {
    pub io_devices: Vec<Box<dyn IODevice>>,
    pub port_map: HashMap<u8, u8>,
    iff1: bool,
    iff2: bool,
}

impl Default for IO {
    fn default() -> IO {
        let registers = IORegister::default();
        let mut port_map = HashMap::new();
        for pin in registers.pins() {
            port_map.insert(pin, 0);
        }
        IO {
            io_devices: vec![Box::new(registers)],
            port_map,
            iff1: false,
            iff2: false,
        }
    }
}

impl IO {
    pub fn read(&self, port: u8) -> Result<u8, &str> {
        let device_id = self.port_map.get(&port).ok_or("Attempting to read from unconnected port")?;
        let device = self.io_devices.get(*device_id as usize).unwrap();
        device.read(port)
    }

    pub fn write(&mut self, port: u8, data: u8) -> Result<(), &str> {
        let device_id = self.port_map.get(&port).unwrap();
        let device = self.io_devices.get_mut(*device_id as usize).unwrap();
        device.write(port, data)
    }

    pub fn step(&mut self) {
        for device in self.io_devices.iter_mut() {
            device.step();
        }
    }

    pub fn add_device(&mut self, device: Box<dyn IODevice>) -> Result<(), &'static str> {
        let pins = device.pins();
        for pin in pins {
            if self.port_map.contains_key(&pin) {
                return Err("Attempting to add a device with a port already in use by other device");
            }
            self.port_map.insert(pin, self.io_devices.len() as u8);
        }
        self.io_devices.push(device);
        Ok(())
    }

    pub fn get_interrupt(&self) -> Option<(InterruptType, usize)> {
        for (i, device) in self.io_devices.iter().enumerate() {
            match device.will_interrupt() {
                Some(InterruptType::NMI) => { return Some((InterruptType::NMI, i)); }
                Some(val) if self.int_enabled() => { return Some((val, i)); }
                _ => {}
            }
        }
        None
    }

    pub fn ack_int(&mut self, device_id: usize) -> Result<(), &str> {
        let device = self.io_devices.get_mut(device_id).ok_or("Attempting to acknowledge interrupt from non existent device")?;
        device.ack_int()
    }

    pub fn int_enabled(&self) -> bool {
        self.iff1
    }
    pub fn enable_int(&mut self) {
        self.iff1 = true;
    }
    pub fn start_int(&mut self) {
        self.iff2 = self.iff1;
    }
    pub fn disable_int(&mut self) {
        self.iff1 = self.iff2;
    }
}