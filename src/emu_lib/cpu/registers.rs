use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct UnionRegister {
    value: u16,
}

impl UnionRegister {
    pub fn new() -> UnionRegister {
        UnionRegister {
            value: 0,
        }
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn get_high(&self) -> u8 {
        (self.value >> 8) as u8
    }

    pub fn get_low(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    pub fn set_high(&mut self, value: u8) {
        self.value = (self.value & 0x00FF) | ((value as u16) << 8);
    }

    pub fn set_low(&mut self, value: u8) {
        self.value = (self.value & 0xFF00) | (value as u16);
    }
}

pub enum AnyRegister {
    Bit8(u8),
    Bit16(UnionRegister),
}

pub trait RegisterOps {
    fn get_all(&self) -> HashMap<&str, AnyRegister>;
    fn get8bit(&self, register: &str) -> u8;
    fn get16bit(&self, register: &str) -> u16;
    fn set8bit(&mut self, register: &str, value: u8);
    fn set16bit(&mut self, register: &str, value: u16);
    fn increment_pc(&mut self, value: u16);
}