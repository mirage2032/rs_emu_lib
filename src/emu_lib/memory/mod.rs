use std::io::Write;
use std::ops::Index;

mod memdevice;

pub struct Memory {
    data: Vec<Box<dyn memdevice::MemDevice>>,
}

impl Memory {
    pub fn new_empty() -> Memory {
        Memory {
            data: Vec::new()
        }
    }

    pub fn new() -> Memory {
        Memory {
            data: vec![Box::new(memdevice::MemBank::new(0x4000)),
                       Box::new(memdevice::MemBank::new(0xC000))],
        }
    }

    pub fn clear(&mut self) {
        for device in &mut self.data {
            device.clear().unwrap();
        }
    }

    pub fn add_device(&mut self, device: Box<dyn memdevice::MemDevice>) {
        self.data.push(device);
    }

    fn get_elem_idx(&self, addr: u16) -> Result<(usize, u16), &str> {
        let mut offset = 0u16;
        for (index, device) in self.data.iter().enumerate() {
            if addr >= offset && addr < offset + device.size() {
                return Ok((index, addr - offset));
            }
            offset += device.size();
        }
        Err("Address not mapped")
    }
    pub fn read(&self, addr: u16) -> u8 {
        if let Ok((device_idx, offset)) = self.get_elem_idx(addr) {
            *self.data[device_idx].read(offset)
        } else {
            0
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);
        (high as u16) << 8 | low as u16
    }
    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), &str> {
        if let Ok((device_idx, offset)) = self.get_elem_idx(addr) {
            self.data[device_idx].write(offset, data)?;
            Ok(())
        } else {
            Err("Address not mapped")
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), &str> {
        let mut file = std::fs::File::create(filename).map_err(|_| "Error creating file")?;
        for device in &self.data {
            for byte in 0..device.size() {
                file.write_all(&[*device.read(byte)]).map_err(|_| "Error writing to file")?;
            }
        }
        Ok(())
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, addr: u16) -> &u8 {
        if let Ok((device_idx, offset)) = self.get_elem_idx(addr)
        {
            &self.data[device_idx][offset]
        } else {
            panic!("Address {:x} not mapped", addr);
        }
    }
}