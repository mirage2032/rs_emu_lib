use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

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
    pub port_map: HashMap<u8, Weak<RefCell<Box<dyn IODevice>>>>,
    devices: Vec<Option<Rc<RefCell<Box<dyn IODevice>>>>>,
    pub iff1: bool,
    pub iff2: bool,
}

impl IO {
    pub fn new() -> IO {
        let registers: Rc<RefCell<Box<dyn IODevice>>> = Rc::new(RefCell::new(Box::new(IORegister::default())));
        let mut port_map = HashMap::new();
        for port in registers.borrow().ports() {
            port_map.insert(port, Rc::downgrade(&registers));
        }
        IO {
            port_map,
            devices: vec![Some(registers)],
            iff1: false,
            iff2: false,
        }
    }
}

impl Default for IO {
    fn default() -> IO {
        let registers: Rc<RefCell<Box<dyn IODevice>>> = Rc::new(RefCell::new(Box::new(IORegister::default())));
        let mut port_map = HashMap::new();
        for port in registers.borrow().ports() {
            port_map.insert(port, Rc::downgrade(&registers));
        }
        IO {
            port_map,
            devices: vec![Some(registers)],
            iff1: false,
            iff2: false,
        }
    }
}

impl IO {
    pub fn read(&self, port: u8) -> Result<u8, &str> {
        let device: Weak<RefCell<Box<dyn IODevice>>> = self.port_map.get(&port).ok_or("Attempting to read from unconnected port")?.clone();
        device.upgrade().ok_or("Attempting to read from removed device")?.borrow().read(port)
    }

    pub fn write(&mut self, port: u8, data: u8) -> Result<(), &str> {
        let device = self
            .port_map
            .get(&port)
            .ok_or("Attempting to write to unconnected port")?;
        device.upgrade().ok_or("Attempting to write to removed device")?.borrow_mut().write(port, data)
    }

    pub fn step(&mut self) {
        for device in self.devices.iter_mut().flatten() {
            device.borrow_mut().step();
        }
    }

    pub fn add_device(&mut self, device: Box<dyn IODevice>) -> Result<(), &'static str> {
        let dev: Rc<RefCell<Box<dyn IODevice>>> = Rc::new(RefCell::new(device));
        let ports = dev.borrow().ports();
        for port in ports {
            if self.port_map.contains_key(&port) {
                return Err(
                    "Attempting to add a device with a port already in use by other device",
                );
            }
            self.port_map.insert(port, Rc::downgrade(&dev));
        }
        self.devices.push(Some(dev));
        Ok(())
    }

    pub fn remove_dev_by_id(&mut self, device_id: usize) -> Result<(), &str> {
        self
            .devices
            .remove(device_id);
        Ok(())
    }
    
    pub fn remove_dev_by_port(&mut self, port: u8) -> Result<(), &str> {
        let device = self
            .port_map
            .get(&port)
            .ok_or("Attempting to remove device from unconnected port")?;
        let mut found = false;
        for (i, dev) in self.devices.iter().enumerate() {
            if let Some(dev) = dev {
                if Rc::ptr_eq(&device.upgrade().unwrap(), dev) {
                    self.devices[i] = None;
                    found = true;
                    break;
                }
            }
        }
        
        if !found {
            return Err("Attempting to remove device from unconnected port");
        }
        Ok(())
    }

    pub fn get_interrupt(&self) -> Option<(InterruptType, usize)> {
        let mut min_im = None;
        for (i, device) in self.devices.iter().enumerate() {
            if let Some(dev) = device {
                match (dev.borrow().will_interrupt(),&min_im) {
                    (Some(InterruptType::NMI),_) => {
                        return  Some((InterruptType::NMI, i));
                    }
                    (Some(val),None) if self.int_enabled() => {
                        min_im = Some((val, i));
                    }
                    _ => {}
                }
            }
        }
        min_im
    }

    pub fn ack_int(&mut self, device_id: usize) -> Result<(), &str> {
        let devopt = self
            .devices
            .get_mut(device_id)
            .ok_or("Attempting to acknowledge interrupt from non existent device")?;
        if let Some(dev) = devopt {
            dev.borrow_mut().ack_int()
        } else {
            Err("Attempting to acknowledge interrupt from removed device")
        }
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
