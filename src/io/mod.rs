use iodevice::IODevice;
use iodevice::IORegister;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

pub mod iodevice;

pub struct IO {
    pub port_map: HashMap<u8, Weak<Mutex<Box<dyn IODevice>>>>,
    devices: Vec<Arc<Mutex<Box<dyn IODevice>>>>,
}

impl Default for IO {
    fn default() -> IO {
        let registers: Arc<Mutex<Box<dyn IODevice>>> =
            Arc::new(Mutex::new(Box::new(IORegister::default())));
        let mut port_map = HashMap::new();
        for port in registers.lock().expect("Failed to get IO lock").ports() {
            port_map.insert(port, Arc::downgrade(&registers));
        }
        IO {
            port_map,
            devices: vec![registers],
        }
    }
}

impl IO {
    pub fn new() -> IO{
        IO{
            port_map: HashMap::new(),
            devices: Vec::new(),
        }
    }
    pub fn read(&self, port: u8) -> Result<u8, &str> {
        // let device: Weak<Mutex<Box<dyn IODevice>>> = self
        //     .port_map
        //     .get(&port)
        //     .ok_or("Attempting to read from unconnected port")?
        //     .clone();
        // device
        //     .upgrade()
        //     .ok_or("Attempting to read from removed device")?
        //     .lock()
        //     .expect("Failed to get IO lock")
        //     .read(port)
        let device: Weak<Mutex<Box<dyn IODevice>>> = self
            .port_map
            .get(&port)
            .ok_or("Attempting to read from unconnected port")?
            .clone();
        device
            .upgrade()
            .ok_or("Attempting to read from removed device")?
            .lock()
            .expect("Failed to get IO lock")
            .read(port)
    }

    pub fn write(&mut self, port: u8, data: u8) -> Result<(), &str> {
        let device = self
            .port_map
            .get(&port)
            .ok_or("Attempting to write to unconnected port")?;
        device
            .upgrade()
            .ok_or("Attempting to write to removed device")?
            .lock()
            .expect("Failed to get IO lock")
            .write(port, data)
    }

    pub fn step(&mut self) {
        for device in self.devices.iter() {
            device.lock().expect("Failed to get IO lock").step();
        }
    }

    pub fn add_device(&mut self, device: Box<dyn IODevice>) -> Result<(), &'static str> {
        let dev: Arc<Mutex<Box<dyn IODevice>>> = Arc::new(Mutex::new(device));
        let ports = dev.lock().expect("Failed to get IO lock").ports();
        for port in ports {
            if self.port_map.contains_key(&port) {
                return Err(
                    "Attempting to add a device with a port already in use by other device",
                );
            }
            self.port_map.insert(port, Arc::downgrade(&dev));
        }
        self.devices.push(dev);
        Ok(())
    }

    pub fn remove_dev_by_id(&mut self, device_id: usize) -> Result<(), &str> {
        self.devices.remove(device_id);
        Ok(())
    }

    pub fn remove_dev_by_port(&mut self, port: u8) -> Result<(), &str> {
        let device = self
            .port_map
            .get(&port)
            .ok_or("Attempting to remove device from unconnected port")?;
        let mut found = false;
        for (i, dev) in self.devices.iter().enumerate() {
            if Arc::ptr_eq(&device.upgrade().unwrap(), dev) {
                self.devices.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            return Err("Attempting to remove device from unconnected port");
        }
        Ok(())
    }

    /// Whether any device is holding INT active.
    pub fn int_pending(&self) -> bool {
        self.devices
            .iter()
            .any(|device| device.lock().expect("Failed to get IO lock").int_pending())
    }

    /// Acknowledges the first device, in the order they were added, that is holding
    /// INT active, and returns the byte it puts on the data bus. `None` if no device
    /// is. Each device is checked and acknowledged under one lock.
    pub fn int_ack(&mut self) -> Option<u8> {
        self.devices.iter().find_map(|device| {
            let mut device = device.lock().expect("Failed to get IO lock");
            if device.int_pending() {
                Some(device.int_ack())
            } else {
                None
            }
        })
    }
}
