use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use bitfield_struct::bitfield;

use crate::emu_lib::cpu::{RegisterOps, SingleRegister};

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct Flags {
    // Define your fields with their sizes
    pub carry: bool,
    pub add_sub: bool,
    pub parity_overflow: bool,
    _bit3: bool,
    pub half_carry: bool,
    _bit5: bool,
    pub zero: bool,
    pub sign: bool,
}

#[cfg(target_endian = "big")]
#[derive(Default, Debug)]
#[repr(C)]
pub struct ByteRegisters {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

#[cfg(target_endian = "little")]
#[derive(Default, Debug)]
#[repr(C)]
pub struct ByteRegisters {
    pub f: Flags,
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

#[derive(Debug)]
pub struct Registers {
    pub main: ByteRegisters,
    pub shadow: ByteRegisters,
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
            shadow: ByteRegisters::default(),
            ix: 0,
            iy: 0,
            sp: 0xFFFF,
            pc: 0,
            i: 0,
            r: 0,
        }
    }
}

impl RegisterOps for Registers {
    fn clear(&mut self) {
        self.main = ByteRegisters::default();
        self.shadow = ByteRegisters::default();
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
            "f" => self.main.f = value.into(),
            "b" => self.main.b = value,
            "c" => self.main.c = value,
            "d" => self.main.d = value,
            "e" => self.main.e = value,
            "h" => self.main.h = value,
            "l" => self.main.l = value,
            "a'" => self.shadow.a = value,
            "f'" => self.shadow.f = value.into(),
            "b'" => self.shadow.b = value,
            "c'" => self.shadow.c = value,
            "d'" => self.shadow.d = value,
            "e'" => self.shadow.e = value,
            "h'" => self.shadow.h = value,
            "l'" => self.shadow.l = value,
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
            "af'" => self.shadow.af = value,
            "bc'" => self.shadow.bc = value,
            "de'" => self.shadow.de = value,
            "hl'" => self.shadow.hl = value,
            "ix" => self.ix = value,
            "iy" => self.iy = value,
            "pc" => self.pc = value,
            "sp" => self.sp = value,
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
        map.insert("f", SingleRegister::Bit8(self.main.f.into()));
        map.insert("b", SingleRegister::Bit8(self.main.b));
        map.insert("c", SingleRegister::Bit8(self.main.c));
        map.insert("d", SingleRegister::Bit8(self.main.d));
        map.insert("e", SingleRegister::Bit8(self.main.e));
        map.insert("h", SingleRegister::Bit8(self.main.h));
        map.insert("l", SingleRegister::Bit8(self.main.l));

        map.insert("af'", SingleRegister::Bit16(self.shadow.af));
        map.insert("bc'", SingleRegister::Bit16(self.shadow.bc));
        map.insert("de'", SingleRegister::Bit16(self.shadow.de));
        map.insert("hl'", SingleRegister::Bit16(self.shadow.hl));
        map.insert("a'", SingleRegister::Bit8(self.shadow.a));
        map.insert("f'", SingleRegister::Bit8(self.shadow.f.into()));
        map.insert("b'", SingleRegister::Bit8(self.shadow.b));
        map.insert("c'", SingleRegister::Bit8(self.shadow.c));
        map.insert("d'", SingleRegister::Bit8(self.shadow.d));
        map.insert("e'", SingleRegister::Bit8(self.shadow.e));
        map.insert("h'", SingleRegister::Bit8(self.shadow.h));
        map.insert("l'", SingleRegister::Bit8(self.shadow.l));

        map.insert("ix", SingleRegister::Bit16(self.ix));
        map.insert("iy", SingleRegister::Bit16(self.iy));
        map.insert("sp", SingleRegister::Bit16(self.sp));
        map.insert("pc", SingleRegister::Bit16(self.pc));
        map.insert("i", SingleRegister::Bit8(self.i));
        map.insert("r", SingleRegister::Bit8(self.r));
        map
    }

    fn pc(&self) -> &u16 {
        &self.pc
    }
    fn pc_mut_ref(&mut self) -> &mut u16 {
        &mut self.pc
    }
}