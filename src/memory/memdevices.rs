use crate::memory::errors::{MemoryRWCommonError, MemoryReadError, MemoryWriteError};
use crate::memory::MemoryDevice;

#[derive(Debug, Clone)]
pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM {
            data: vec![0; size],
        }
    }
}

impl MemoryDevice for RAM {
    fn size(&self) -> usize {
        self.data.len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        match self.data.get(addr as usize) {
            None => Err(MemoryRWCommonError::OutOfBounds(addr).into()),
            Some(val) => Ok(*val),
        }
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        let val = self
            .data
            .get_mut(addr as usize)
            .ok_or(MemoryRWCommonError::OutOfBounds(addr))?;
        *val = data;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        self.write_8(addr, data)
    }
}

impl From<Vec<u8>> for RAM {
    fn from(data: Vec<u8>) -> RAM {
        RAM { data }
    }
}

#[derive(Debug, Clone)]
pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new(size: usize) -> ROM {
        ROM {
            data: vec![0; size],
        }
    }
}

impl MemoryDevice for ROM {
    fn size(&self) -> usize {
        self.data.len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        match self.data.get(addr as usize) {
            None => Err(MemoryRWCommonError::OutOfBounds(addr).into()),
            Some(val) => Ok(*val),
        }
    }
    fn write_8(&mut self, addr: u16, _: u8) -> Result<(), MemoryWriteError> {
        Err(MemoryWriteError::ReadOnly(addr))
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        let val = self
            .data
            .get_mut(addr as usize)
            .ok_or(MemoryRWCommonError::OutOfBounds(addr))?;
        *val = data;
        Ok(())
    }
}

impl From<Vec<u8>> for ROM {
    fn from(data: Vec<u8>) -> ROM {
        ROM { data }
    }
}
