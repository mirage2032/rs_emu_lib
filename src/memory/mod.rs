use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use errors::{FileError, MemWriteError, MemoryError};

pub mod errors;
pub mod memdevices;

pub struct Memory {
    data: Vec<Box<dyn MemoryDevice>>,
    changes: Option<Vec<u16>>,
    readcallback: Option<fn(u16, u8)>,
    writecallback: Option<fn(u16, u8)>,
}

pub trait MemoryDevice {
    fn size(&self) -> usize;
    fn read_8(&self, addr: u16) -> Result<u8, &'static str>;
    fn read_16(&self, addr: u16) -> Result<u16, &'static str> {
        let lsb = self.read_8(addr)?;
        let msb = self.read_8(addr.wrapping_add(1))?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str>;
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), &'static str> {
        let bytes = data.to_le_bytes();
        self.write_8(addr, bytes[0])?;
        self.write_8(addr.wrapping_add(1), bytes[1])?;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), &'static str>;
    fn write_16_force(&mut self, addr: u16, data: u16) -> Result<(), &'static str> {
        let bytes = data.to_le_bytes();
        self.write_8_force(addr, bytes[0])?;
        self.write_8_force(addr.wrapping_add(1), bytes[1])?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), &'static str> {
        for i in 0..self.size() {
            self.write_8(i as u16, 0)?;
        }
        Ok(())
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: Vec::new(),
            changes: None,
            writecallback: None,
            readcallback: None,
        }
    }

    pub fn add_device(&mut self, device: Box<dyn MemoryDevice>) {
        self.data.push(device);
    }

    pub fn record_changes(&mut self, active: bool) {
        if active {
            self.changes = Some(Vec::new());
        } else {
            self.changes = None;
        }
    }
    pub fn clear_changes(&mut self) {
        if let Some(changes) = &mut self.changes {
            changes.clear();
        }
    }
    pub fn add_write_callback(&mut self, callback: Option<fn(u16, u8)>) {
        self.writecallback = callback;
    }

    pub fn add_read_callback(&mut self, callback: Option<fn(u16, u8)>) {
        self.readcallback = callback;
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

    pub fn save(&self) -> Result<Vec<u8>, MemoryError> {
        let mut data = Vec::new();
        for device in &self.data {
            for byte in 0..device.size() {
                data.push(
                    device
                        .read_8(byte as u16)
                        .map_err(|err| MemoryError::MemRead(byte, err))?,
                );
            }
        }
        Ok(data)
    }

    pub fn save_file(&self, filename: PathBuf) -> Result<(), MemoryError> {
        if fs::metadata(&filename).is_ok() {
            return Err(FileError::FileExists(filename).into());
        }
        fs::write(&filename, &self.save()?).map_err(|_| FileError::FileCreate(filename))?;
        Ok(())
    }

    pub fn load(&mut self, data: &[u8]) -> Result<(), Vec<MemoryError>> {
        let mut result: Vec<MemoryError> = vec![];
        let mut index: usize = 0;
        for device in &mut self.data {
            let mut offset = 0;
            let dev_size = device.size();
            while offset < dev_size {
                if index >= data.len() {
                    break;
                }
                let byte = data[index];
                if let Err(err) = device.write_8_force(offset as u16, byte) {
                    result.push(MemWriteError::Write(index, err).into());
                }
                offset += 1;
                index += 1;
            }
        }
        if result.is_empty() {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn load_file(&mut self, filename: &Path) -> Result<(), Vec<MemoryError>> {
        if fs::metadata(filename).is_err() {
            return Err(vec![
                FileError::FileDoesNotExist(filename.to_path_buf()).into()
            ]);
        }
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                self.load(&reader.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>())
            }
            Err(_) => Err(vec![FileError::FileCreate(filename.to_path_buf()).into()]),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut mem = Self::new();
        mem.add_device(Box::new(memdevices::RAM::new(0x4000)));
        mem.add_device(Box::new(memdevices::RAM::new(0xC000)));
        mem
    }
}

impl MemoryDevice for Memory {
    fn size(&self) -> usize {
        self.data.iter().map(|d| d.size()).sum()
    }
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        let data = self.data[device_idx].read_8(offset as u16)?;
        if let Some(callback) = &self.readcallback {
            callback(addr, data);
        }
        Ok(data)
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write_8(offset as u16, data)?;
        if let Some(callback) = &self.writecallback {
            callback(addr, data);
        }
        if let Some(changes) = &mut self.changes {
            changes.push(addr);
        }
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        self.data[device_idx].write_8_force(offset as u16, data)?;
        if let Some(callback) = &self.writecallback {
            callback(addr, data);
        }
        if let Some(changes) = &mut self.changes {
            changes.push(addr);
        }
        Ok(())
    }
}
