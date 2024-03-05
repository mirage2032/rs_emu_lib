use std::collections::HashMap;

use iodevice::IODevice;
use iodevice::IORegister;
use crate::io::iodevice::InterruptType;

pub mod iodevice;

pub struct IO {
    pub io_devices: Vec<Box<dyn IODevice>>,
    pub port_map: HashMap<u8, u8>,
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

    pub fn add_device(&mut self, device: Box<dyn IODevice>) {
        let pins = device.pins();
        for pin in pins {
            if self.port_map.contains_key(&pin) {
                panic!("Attempting to add device with port already in use by other device");
            }
            self.port_map.insert(pin, self.io_devices.len() as u8);
        }
        self.io_devices.push(device);
    }

    pub fn will_interrupt(&self) -> Option<InterruptType> {
        for device in self.io_devices.iter() {
            match device.will_interrupt() {
                None => continue,
                val => { return val; }
            }
        }
        None
    }
}