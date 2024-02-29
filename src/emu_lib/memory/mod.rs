use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::ops::Index;

mod memdevice;

pub struct Memory {
    data: Vec<Box<dyn memdevice::MemDevice>>,
}

#[derive(Debug)]
pub enum MemoryError {
    OpenFile,
    ReadError,
    ReadOnly(usize, u32),
    UnmappedAddress(u32),
}

pub trait ReadableMemory {
    fn read_8(&self, addr: u16) -> Result<&u8, String>;
    fn read_16(&self, addr: u16) -> Result<u16, String>;
}

pub trait WritableMemory {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), String>;
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), String>;
}

impl ReadableMemory for Vec<u8> {
    fn read_8(&self, addr: u16) -> Result<&u8, String> {
        Ok(&self[addr as usize])
    }

    fn read_16(&self, addr: u16) -> Result<u16, String> {
        let lsb = self[addr as usize];
        let msb = self[(addr + 1) as usize];
        Ok(u16::from_le_bytes([lsb, msb]))
    }
}

impl WritableMemory for Vec<u8> {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), String> {
        self[addr as usize] = data;
        Ok(())
    }
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), String> {
        let bytes = data.to_le_bytes();
        if addr + 1 >= self.len() as u16 {
            return Err("Address out of bounds".to_string());
        }
        self[addr as usize] = bytes[0];
        self[(addr + 1) as usize] = bytes[1];
        Ok(())
    }
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
            let device_end = offset + (device.size() - 1);
            if addr >= offset && addr < device_end {
                return Ok((index, addr - offset)); // Return the index and the offset
            }
            offset += device.size();
        }
        Err("Address not mapped")
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
                let filesize = file.metadata().unwrap().len();
                let mut result = vec![];
                let mut reader = BufReader::new(file);
                let mut index: u32 = 0;
                for device in &mut self.data {
                    if device.is_read_only() {
                        result.push(MemoryError::ReadOnly(index as usize, device.size() as u32));
                    } else {
                        match reader.read_exact(device.data_mut()) {
                            Ok(_) => {
                                index += device.size() as u32;
                            }
                            Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                                index+= device.size() as u32;
                                break;
                            }
                            Err(_) => {
                                result.push(MemoryError::ReadError);
                                break;
                            }
                        }
                    }
                }
                if filesize >= index as u64 {
                    result.push(MemoryError::UnmappedAddress(index));
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

impl ReadableMemory for Memory {
    fn read_8(&self, addr: u16) -> Result<&u8, String> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        Ok(self.data[device_idx].read(offset))
    }

    fn read_16(&self, addr: u16) -> Result<u16, String> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        let lsb = *self.data[device_idx].read(offset);
        let msb = *self.data[device_idx].read(offset + 1);
        Ok(u16::from_le_bytes([lsb, msb]))
    }
}

impl WritableMemory for Memory {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), String> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write(offset, data)?;
        Ok(())
    }
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), String> {
        let bytes = data.to_le_bytes();
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write(offset, bytes[0])?;
        self.data[device_idx].write(offset, bytes[1])?;
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