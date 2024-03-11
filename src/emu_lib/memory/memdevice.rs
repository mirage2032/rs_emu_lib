use crate::emu_lib::memory::{ReadableMemory, RWMemory, WriteableMemory};
use crate::emu_lib::utils::Size;

pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize, ro: bool) -> RAM {
        RAM {
            data: vec![0; size],
        }
    }
}

impl Size for RAM {
    fn size(&self) -> usize {
        self.data.len()
    }
}

impl ReadableMemory for RAM {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        let val = self.data.get(addr as usize).ok_or("Address out of bounds")?;
        Ok(*val)
    }
}

impl WriteableMemory for RAM {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        let val = self.data.get_mut(addr as usize).ok_or("Address out of bounds")?;
        *val = data;
        Ok(())
    }
}

impl RWMemory for RAM {}

pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new(size: usize, ro: bool) -> ROM {
        ROM {
            data: vec![0; size],
        }
    }
}

impl Size for ROM {
    fn size(&self) -> usize {
        self.data.len()
    }
}

impl ReadableMemory for ROM {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        let val = self.data.get(addr as usize).ok_or("Address out of bounds")?;
        Ok(*val)
    }
}

impl WriteableMemory for ROM {
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        Err("ROM is read only")
    }
}

impl RWMemory for ROM {}

