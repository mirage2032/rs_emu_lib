use std::ops::Index;

pub trait MemDevice: Send + Sync {
    fn size(&self) -> u16;
    fn read(&self, addr: u16) -> &u8;
    fn data_mut(&mut self) -> &mut Vec<u8>;
    fn write(&mut self, addr: u16, data: u8) -> Result<(), &str>;
    fn is_read_only(&self) -> bool;
    fn clear(&mut self) -> Result<(), &str> {
        if self.is_read_only() {
            return Err("Can't clear read-only memory");
        }
        for i in 0..self.size() {
            self.write(i, 0).unwrap();
        }
        Ok(())
    }
}

impl Index<u16> for dyn MemDevice {
    type Output = u8;

    fn index(&self, index: u16) -> &u8 {
        self.read(index)
    }
}

pub struct MemBank {
    data: Vec<u8>,
    read_only: bool,
}

impl MemBank {
    pub fn new(size: usize, ro: bool) -> MemBank {
        MemBank {
            data: vec![0; size],
            read_only: ro,
        }
    }
}

impl Index<u8> for MemBank {
    type Output = u8;

    fn index(&self, index: u8) -> &u8 {
        &self.data[index as usize]
    }
}

impl MemDevice for MemBank {
    fn size(&self) -> u16 {
        self.data.len() as u16
    }
    fn read(&self, addr: u16) -> &u8 {
        &self.data[addr as usize]
    }

    fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    fn write(&mut self, addr: u16, data: u8) -> Result<(), &str> {
        if self.read_only {
            return Err("Write to read-only memory");
        }
        self.data[addr as usize] = data;
        Ok(())
    }

    fn is_read_only(&self) -> bool {
        self.read_only
    }
}