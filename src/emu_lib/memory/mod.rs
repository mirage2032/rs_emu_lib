use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

pub use memdevice::RAM;

use crate::utils::Size;

mod memdevice;

pub struct Memory {
    data: Vec<Box<dyn MemoryDevice>>,
}

#[derive(Debug)]
pub enum MemoryError {
    FileError,
    ReadError,
    ReadOnly(usize, usize),
    EndOfMem(usize),
}

pub trait MemoryDevice: Size {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str>;
    fn read_16(&self, addr: u16) -> Result<u16, &'static str> {
        let lsb = self.read_8(addr)?;
        let msb = self.read_8(addr + 1)?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str>;
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), &'static str> {
        let bytes = data.to_le_bytes();
        self.write_8(addr, bytes[0])?;
        self.write_8(addr + 1, bytes[1])?;
        Ok(())
    }
    fn clear(&mut self) -> Result<(), &'static str> {
        for i in 0..self.size() {
            self.write_8(i as u16, 0)?;
        }
        Ok(())
    }
}

impl Size for Vec<u8> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl MemoryDevice for Vec<u8> {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        Ok(self[addr as usize])
    }
    fn read_16(&self, addr: u16) -> Result<u16, &'static str> {
        let lsb = self.read_8(addr)?;
        let msb = self.read_8(addr + 1)?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        self[addr as usize] = data;
        Ok(())
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn add_device(&mut self, device: Box<dyn MemoryDevice>) {
        self.data.push(device);
    }

    fn get_elem_idx(&self, addr: u16) -> Result<(usize, usize), &'static str> {
        let mut offset = 0;
        for (index, device) in self.data.iter().enumerate() {
            let dev_size = device.size();
            let device_end = offset + dev_size;
            if addr as usize >= offset && (addr as usize) < device_end {
                return Ok((index, addr as usize - offset)); // Return the index and the offset
            }
            offset += dev_size;
        }
        Err("Address not mapped")
    }

    pub fn save(&self, filename: String) -> Result<(), &'static str> {
        let mut file = File::create(filename).map_err(|_| "Error creating file")?;
        for device in &self.data {
            for byte in 0..device.size() {
                file.write_all(&[device.read_8(byte as u16)?]).map_err(|_| "Error writing to file")?;
            }
        }
        Ok(())
    }

    pub fn load(&mut self, filename: String) -> Result<(), Vec<MemoryError>> {
        match File::open(filename) {
            Ok(file) => {
                let filesize = file.metadata().unwrap().len();
                let mut result = vec![];
                let mut reader = BufReader::new(file);
                let mut index: usize = 0;
                for device in &mut self.data {
                    // if device.is_read_only() {
                    //     let dev_size = device.size();
                    //     reader.consume(dev_size);
                    //     index += device.size();
                    //     result.push(MemoryError::ReadOnly(index, device.size()));
                    // } else {
                    let mut buffer = vec![0; device.size()];
                    match reader.read_exact(&mut buffer) {
                        Ok(_) => {
                            for (i, byte) in buffer.iter().enumerate() {
                                device.write_8(i as u16, *byte).unwrap();
                            }
                            index += device.size();
                        }
                        Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                            for (i, byte) in buffer.iter().enumerate() {
                                device.write_8(i as u16, *byte).unwrap();
                            }
                            index += device.size();
                        }
                        Err(_) => {
                            result.push(MemoryError::ReadError);
                            break;
                        }
                        // }
                    }
                }
                if filesize >= index as u64 {
                    result.push(MemoryError::EndOfMem(index));
                }

                if result.is_empty() {
                    Ok(())
                } else {
                    Err(result)
                }
            }
            Err(_) => Err(vec![MemoryError::FileError]),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut mem = Self::new();
        mem.add_device(Box::new(memdevice::RAM::new(0x4000)));
        mem.add_device(Box::new(memdevice::ROM::new(0xC000)));
        mem
    }
}

impl Size for Memory {
    fn size(&self) -> usize {
        self.data.iter().map(|d| d.size()).sum()
    }
}

impl MemoryDevice for Memory {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].read_8(offset as u16)
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write_8(offset as u16, data)
    }
}