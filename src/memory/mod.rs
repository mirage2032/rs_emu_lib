use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use errors::{FileError, MemoryRWError};
use crate::memory::errors::{MemoryRWCommonError, MemoryReadError, MemorySaveLoadError, MemoryWriteError};

pub mod errors;
pub mod memdevices;

pub struct Memory {
    data: Vec<Box<dyn MemoryDevice>>,
    changes: Option<Vec<u16>>,
    readcallback: Option<fn(u16, u8)>,
    writecallback: Option<fn(u16, u8)>,
}

pub trait MemoryDevice: Send + Sync{
    fn size(&self) -> usize;
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError>;
    fn read_16(&self, addr: u16) -> Result<u16, MemoryReadError> {
        let lsb = self.read_8(addr)?;
        let msb = self.read_8(addr.wrapping_add(1))?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError>;
    fn write_16(&mut self, addr: u16, data: u16) -> Result<(), MemoryWriteError> {
        let bytes = data.to_le_bytes();
        self.write_8(addr, bytes[0])?;
        self.write_8(addr.wrapping_add(1), bytes[1])?;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError>;
    fn write_16_force(&mut self, addr: u16, data: u16) -> Result<(), MemoryWriteError> {
        let bytes = data.to_le_bytes();
        self.write_8_force(addr, bytes[0])?;
        self.write_8_force(addr.wrapping_add(1), bytes[1])?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), MemoryWriteError> {
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

    pub fn new_full_ram() -> Memory {
        let mut mem = Memory::new();
        mem.add_device(Box::new(memdevices::RAM::new(0x10000)));
        mem
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

    fn get_elem_idx(&self, addr: u16) -> Result<(usize, usize), MemoryRWCommonError> {
        let mut offset = 0;
        for (index, device) in self.data.iter().enumerate() {
            let dev_size = device.size();
            let device_end = offset + dev_size;
            if addr as usize >= offset && (addr as usize) < device_end {
                return Ok((index, addr as usize - offset)); // Return the index and the offset
            }
            offset += dev_size;
        }
        Err(MemoryRWCommonError::UnmappedAddress(addr))
    }

    pub fn save(&self) -> Result<Vec<u8>, MemoryReadError> {
        let mut data = Vec::new();
        for device in &self.data {
            for byte in 0..device.size() {
                data.push(
                    device
                        .read_8(byte as u16)?,
                );
            }
        }
        Ok(data)
    }

    pub fn save_file(&self, filename: PathBuf) -> Result<(), MemorySaveLoadError> {
        if fs::metadata(&filename).is_ok() {
            return Err(FileError::FileExists(filename).into());
        }
        fs::write(
            &filename, 
            &self.save().map_err(MemoryRWError::MemRead)?
        )
            .map_err(|_| FileError::FileCreate(filename))?;
        Ok(())
    }

    pub fn load(&mut self, data: &[u8],force:bool) -> Result<(), Vec<MemoryWriteError>> {
        let write_8 = match force {
            true => |device:&mut Box<dyn MemoryDevice>,offset,byte| device.write_8_force(offset,byte),
            false => |device:&mut Box<dyn MemoryDevice>,offset,byte| device.write_8(offset,byte),
        };
        let mut result: Vec<MemoryWriteError> = vec![];
        let mut index: usize = 0;
        for device in &mut self.data {
            let mut offset = 0;
            let dev_size = device.size();
            while offset < dev_size {
                if index >= data.len() {
                    break;
                }
                let byte = data[index];
                if let Err(err) = write_8(device,offset as u16, byte) {
                    result.push(err);
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

    pub fn load_file(&mut self, filename: &Path,force:bool) -> Result<(), Vec<MemorySaveLoadError>> {
        if fs::metadata(filename).is_err() {
            return Err(vec![
                FileError::FileDoesNotExist(filename.to_path_buf()).into()
            ]);
        }
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                self.load(
                    &reader.bytes().map(
                        |b|
                            b.unwrap()
                    ).collect::<Vec<u8>>(),
                    force
                ).map_err(|e| e.iter().map(|e| MemoryRWError::MemWrite(e.clone()).into()).collect())
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
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        let (device_idx, offset) = self.get_elem_idx(addr)?;
        let data = self.data[device_idx].read_8(offset as u16)?;
        if let Some(callback) = &self.readcallback {
            callback(addr, data);
        }
        Ok(data)
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
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

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
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

impl MemoryDevice for Vec<u8> {
    fn size(&self) -> usize {
        self.len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        match self.get(addr as usize) {
            Some(val) => Ok(*val),
            None => Err(MemoryRWCommonError::OutOfBounds(addr).into()),
        }
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        let val = self
            .get_mut(addr as usize)
            .ok_or(MemoryRWCommonError::OutOfBounds(addr))?;
        *val = data;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        self.write_8(addr, data)
    }
}