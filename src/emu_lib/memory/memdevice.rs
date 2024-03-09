pub trait MemDevice: Send {
    fn size(&self) -> usize;
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8) -> Result<(), &str>;
    fn is_read_only(&self) -> bool;
    fn clear(&mut self) -> Result<(), &str> {
        if self.is_read_only() {
            return Err("Can't clear read-only memory");
        }
        for i in 0..self.size() {
            self.write(i as u16, 0).unwrap();
        }
        Ok(())
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

impl MemDevice for MemBank {
    fn size(&self) -> usize {
        self.data.len() 
    }
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
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