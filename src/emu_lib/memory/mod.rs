use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::ops::Index;

mod memdevice;

pub struct Memory {
    data: Vec<Box<dyn memdevice::MemDevice>>,
}

pub enum MemoryError {
    OpenFile,
    ReadError,
    ReadOnly(usize, u16),
    NotMapped(u16),
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
    pub fn read8(&self, addr: u16) -> u8 {
        if let Ok((device_idx, offset)) = self.get_elem_idx(addr) {
            *self.data[device_idx].read(offset)
        } else {
            0
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr + 1);
        (high as u16) << 8 | low as u16
    }
    pub fn write8(&mut self, addr: u16, data: u8) -> Result<(), String> {
        if let Ok((device_idx, offset)) = self.get_elem_idx(addr) {
            self.data[device_idx].write(offset, data)?;
            Ok(())
        } else {
            Err("Address not mapped".to_string())
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), &str> {
        let mut file = File::create(filename).map_err(|_| "Error creating file")?;
        for device in &self.data {
            for byte in 0..device.size() {
                file.write_all(&[*device.read(byte)]).map_err(|_| "Error writing to file")?;
            }
        }
        Ok(())
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Vec<MemoryError>> {
        match File::open(filename) {
            Ok(file) => {
                let mut result = vec![];
                let mut reader = BufReader::new(file);
                let mut index: u16 = 0;
                for device in &mut self.data {
                    if device.is_read_only() {
                        result.push(MemoryError::ReadOnly(index as usize, device.size()));
                    } else {
                        match reader.read_exact(device.data_mut()) {
                            Ok(_) => {
                                index += device.size();
                            }
                            Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                                break;
                            }
                            Err(_) => {
                                result.push(MemoryError::ReadError);
                                break;
                            }
                        }
                    }
                }
                if result.is_empty() {
                    Ok(())
                } else {
                    Err(result)
                }
            }
            Err(_) => Err(vec![MemoryError::OpenFile]),
        }
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