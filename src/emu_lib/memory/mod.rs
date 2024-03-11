use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};

pub use memdevice::{MemBank, MemDevice};

mod memdevice;

pub struct Memory {
    data: Vec<Box<dyn memdevice::MemDevice>>,
}

#[derive(Debug)]
pub enum MemoryError {
    OpenFile,
    ReadError,
    ReadOnly(usize, usize),
    UnmappedAddress(usize),
}

pub trait ReadableMemory {
    fn size(&self) -> usize;
    fn read_8(&self, addr: u16) -> Result<u8, String>;
    fn read_16(&self, addr: u16) -> Result<u16, String>;
}

pub trait WriteableMemory {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), String>;
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), String>;
}

impl ReadableMemory for Vec<u8> {
    fn size(&self) -> usize {
        self.len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, String> {
        Ok(self[addr as usize])
    }

    fn read_16(&self, addr: u16) -> Result<u16, String> {
        let lsb = self[addr as usize];
        let msb = self[(addr + 1) as usize];
        Ok(u16::from_le_bytes([lsb, msb]))
    }
}

impl WriteableMemory for Vec<u8> {
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
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn clear(&mut self) {
        for device in &mut self.data {
            device.clear().unwrap();
        }
    }

    pub fn add_device(&mut self, device: Box<dyn memdevice::MemDevice>) {
        self.data.push(device);
    }

    fn get_elem_idx(&self, addr: u16) -> Result<(usize, usize), &str> {
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

    pub fn save(&self, filename: &str) -> Result<(), &str> {
        let mut file = File::create(filename).map_err(|_| "Error creating file")?;
        for device in &self.data {
            for byte in 0..device.size() {
                file.write_all(&[device.read(byte as u16)]).map_err(|_| "Error writing to file")?;
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
                let mut index: usize = 0;
                for device in &mut self.data {
                    if device.is_read_only() {
                        let dev_size = device.size();
                        reader.consume(dev_size);
                        index += device.size();
                        result.push(MemoryError::ReadOnly(index, device.size()));
                    } else {
                        let mut buffer = vec![0; device.size()];
                        match reader.read_exact(&mut buffer) {
                            Ok(_) => {
                                for (i, byte) in buffer.iter().enumerate() {
                                    device.write(i as u16, *byte).unwrap();
                                }
                                index += device.size();
                            }
                            Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                                for (i, byte) in buffer.iter().enumerate() {
                                    device.write(i as u16, *byte).unwrap();
                                }
                                index += device.size();
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

impl Default for Memory {
    fn default() -> Self {
        let mut mem = Self::new();
        mem.add_device(Box::new(memdevice::MemBank::new(0x4000, false)));
        mem.add_device(Box::new(memdevice::MemBank::new(0xC000, true)));
        mem
    }
}

impl ReadableMemory for Memory {
    fn size(&self) -> usize {
        self.data.iter().map(|d| d.size()).sum()
    }
    fn read_8(&self, addr: u16) -> Result<u8, String> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        Ok(self.data[device_idx].read(offset as u16))
    }

    fn read_16(&self, addr: u16) -> Result<u16, String> {
        let lsb = self.read_8(addr)?;
        let msb = self.read_8(addr + 1)?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }
}

impl WriteableMemory for Memory {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), String> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write(offset as u16, data)?;
        Ok(())
    }
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), String> {
        let bytes = data.to_le_bytes();
        self.write_8(addr, bytes[0])?;
        self.write_8(addr, bytes[1])?;
        Ok(())
    }
}