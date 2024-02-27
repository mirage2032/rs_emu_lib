use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::emu_lib::cpu::{RegisterOps, SingleRegister};


#[cfg(target_endian = "big")]
#[derive(Default)]
#[repr(C)]
pub struct ByteRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

#[cfg(target_endian = "little")]
#[derive(Default)]
#[repr(C)]
pub struct ByteRegisters {
    pub f: u8,
    pub a: u8,
    pub c: u8,
    pub b: u8,
    pub e: u8,
    pub d: u8,
    pub l: u8,
    pub h: u8,
}

#[derive(Default)]
#[repr(C)]
pub struct WordRegisters {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
}

impl Deref for ByteRegisters {
    type Target = WordRegisters;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl DerefMut for ByteRegisters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

pub(crate) struct Registers {
    pub main: ByteRegisters,
    pub alt: ByteRegisters,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub i: u8,
    pub r: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            main: ByteRegisters::default(),
            alt: ByteRegisters::default(),
            ix: 0,
            iy: 0,
            sp: 0,
            pc: 0,
            i: 0,
            r: 0,
        }
    }
}

impl RegisterOps for Registers {
    fn clear(&mut self) {
        self.main = ByteRegisters::default();
        self.alt = ByteRegisters::default();
        self.ix = 0;
        self.iy = 0;
        self.sp = 0;
        self.pc = 0;
        self.i = 0;
        self.r = 0;
    }
    fn set_8(&mut self, register: &str, value: u8) {
        match register {
            "a" => self.main.a = value,
            "f" => self.main.f = value,
            "b" => self.main.b = value,
            "c" => self.main.c = value,
            "d" => self.main.d = value,
            "e" => self.main.e = value,
            "h" => self.main.h = value,
            "l" => self.main.l = value,
            "a'" => self.alt.a = value,
            "f'" => self.alt.f = value,
            "b'" => self.alt.b = value,
            "c'" => self.alt.c = value,
            "d'" => self.alt.d = value,
            "e'" => self.alt.e = value,
            "h'" => self.alt.h = value,
            "l'" => self.alt.l = value,
            "i" => self.i = value,
            "r" => self.r = value,
            _ => panic!("Invalid register"),
        }
    }

    fn set_16(&mut self, register: &str, value: u16) {
        match register {
            "af" => self.main.af = value,
            "bc" => self.main.bc = value,
            "de" => self.main.de = value,
            "hl" => self.main.hl = value,
            "af'" => self.alt.af = value,
            "bc'" => self.alt.bc = value,
            "de'" => self.alt.de = value,
            "hl'" => self.alt.hl = value,
            "ix" => self.ix = value,
            "iy" => self.iy = value,
            "sp" => self.sp = value,
            "pc" => self.pc = value,
            _ => panic!("Invalid register"),
        }
    }

    fn get_all(&self) -> HashMap<&str, SingleRegister> {
        let mut map = HashMap::new();
        map.insert("af", SingleRegister::Bit16(self.main.af));
        map.insert("bc", SingleRegister::Bit16(self.main.bc));
        map.insert("de", SingleRegister::Bit16(self.main.de));
        map.insert("hl", SingleRegister::Bit16(self.main.hl));
        map.insert("a", SingleRegister::Bit8(self.main.a));
        map.insert("f", SingleRegister::Bit8(self.main.f));
        map.insert("b", SingleRegister::Bit8(self.main.b));
        map.insert("c", SingleRegister::Bit8(self.main.c));
        map.insert("d", SingleRegister::Bit8(self.main.d));
        map.insert("e", SingleRegister::Bit8(self.main.e));
        map.insert("h", SingleRegister::Bit8(self.main.h));
        map.insert("l", SingleRegister::Bit8(self.main.l));

        map.insert("af'", SingleRegister::Bit16(self.alt.af));
        map.insert("bc'", SingleRegister::Bit16(self.alt.bc));
        map.insert("de'", SingleRegister::Bit16(self.alt.de));
        map.insert("hl'", SingleRegister::Bit16(self.alt.hl));
        map.insert("a'", SingleRegister::Bit8(self.alt.a));
        map.insert("f'", SingleRegister::Bit8(self.alt.f));
        map.insert("b'", SingleRegister::Bit8(self.alt.b));
        map.insert("c'", SingleRegister::Bit8(self.alt.c));
        map.insert("d'", SingleRegister::Bit8(self.alt.d));
        map.insert("e'", SingleRegister::Bit8(self.alt.e));
        map.insert("h'", SingleRegister::Bit8(self.alt.h));
        map.insert("l'", SingleRegister::Bit8(self.alt.l));

        map.insert("ix", SingleRegister::Bit16(self.ix));
        map.insert("iy", SingleRegister::Bit16(self.iy));
        map.insert("sp", SingleRegister::Bit16(self.sp));
        map.insert("pc", SingleRegister::Bit16(self.pc));
        map.insert("i", SingleRegister::Bit8(self.i));
        map.insert("r", SingleRegister::Bit8(self.r));
        map
    }

    fn pc(&mut self) -> &mut u16 {
        &mut self.pc
    }
}