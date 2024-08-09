use std::collections::HashMap;

use crate::emu_lib::cpu::{
    RegisterOps,
    registers::{GPByteRegisters, GPRegister},
};

#[derive(Debug, Default, Clone)]
pub struct Registers {
    pub main: GPByteRegisters,
    pub shadow: GPByteRegisters,
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
            main: GPByteRegisters::default(),
            shadow: GPByteRegisters::default(),
            ix: 0,
            iy: 0,
            sp: 0xFFFF,
            pc: 0,
            i: 0,
            r: 0,
        }
    }
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.main, &mut self.shadow);
    }
}

impl RegisterOps for Registers {
    fn clear(&mut self) {
        self.main = GPByteRegisters::default();
        self.shadow = GPByteRegisters::default();
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

    fn get_8(&self, register: &str) -> u8 {
        match register {
            "a" => self.main.a,
            "f" => self.main.f.into(),
            "b" => self.main.b,
            "c" => self.main.c,
            "d" => self.main.d,
            "e" => self.main.e,
            "h" => self.main.h,
            "l" => self.main.l,
            "a'" => self.shadow.a,
            "f'" => self.shadow.f.into(),
            "b'" => self.shadow.b,
            "c'" => self.shadow.c,
            "d'" => self.shadow.d,
            "e'" => self.shadow.e,
            "h'" => self.shadow.h,
            "l'" => self.shadow.l,
            "i" => self.i,
            "r" => self.r,
            _ => panic!("Invalid register"),
        }
    }
    fn get_16(&self, register: &str) -> u16 {
        match register {
            "af" => self.main.af,
            "bc" => self.main.bc,
            "de" => self.main.de,
            "hl" => self.main.hl,
            "af'" => self.shadow.af,
            "bc'" => self.shadow.bc,
            "de'" => self.shadow.de,
            "hl'" => self.shadow.hl,
            "ix" => self.ix,
            "iy" => self.iy,
            "pc" => self.pc,
            "sp" => self.sp,
            _ => panic!("Invalid register"),
        }
    }

    fn get_all(&self) -> HashMap<&str, GPRegister> {
        let mut map = HashMap::new();
        map.insert("af", GPRegister::Bit16(self.main.af));
        map.insert("bc", GPRegister::Bit16(self.main.bc));
        map.insert("de", GPRegister::Bit16(self.main.de));
        map.insert("hl", GPRegister::Bit16(self.main.hl));
        map.insert("a", GPRegister::Bit8(self.main.a));
        map.insert("f", GPRegister::Bit8(self.main.f.into()));
        map.insert("b", GPRegister::Bit8(self.main.b));
        map.insert("c", GPRegister::Bit8(self.main.c));
        map.insert("d", GPRegister::Bit8(self.main.d));
        map.insert("e", GPRegister::Bit8(self.main.e));
        map.insert("h", GPRegister::Bit8(self.main.h));
        map.insert("l", GPRegister::Bit8(self.main.l));

        map.insert("af'", GPRegister::Bit16(self.shadow.af));
        map.insert("bc'", GPRegister::Bit16(self.shadow.bc));
        map.insert("de'", GPRegister::Bit16(self.shadow.de));
        map.insert("hl'", GPRegister::Bit16(self.shadow.hl));
        map.insert("a'", GPRegister::Bit8(self.shadow.a));
        map.insert("f'", GPRegister::Bit8(self.shadow.f.into()));
        map.insert("b'", GPRegister::Bit8(self.shadow.b));
        map.insert("c'", GPRegister::Bit8(self.shadow.c));
        map.insert("d'", GPRegister::Bit8(self.shadow.d));
        map.insert("e'", GPRegister::Bit8(self.shadow.e));
        map.insert("h'", GPRegister::Bit8(self.shadow.h));
        map.insert("l'", GPRegister::Bit8(self.shadow.l));

        map.insert("ix", GPRegister::Bit16(self.ix));
        map.insert("iy", GPRegister::Bit16(self.iy));
        map.insert("sp", GPRegister::Bit16(self.sp));
        map.insert("pc", GPRegister::Bit16(self.pc));
        map.insert("i", GPRegister::Bit8(self.i));
        map.insert("r", GPRegister::Bit8(self.r));
        map
    }

    fn pc(&self) -> &u16 {
        &self.pc
    }
    fn pc_mut(&mut self) -> &mut u16 {
        &mut self.pc
    }
    fn sp(&self) -> &u16 {
        &self.sp
    }
    fn sp_mut(&mut self) -> &mut u16 {
        &mut self.sp
    }
    fn get_gp(&self) -> &GPByteRegisters {
        &self.main
    }

    fn get_gp_mut(&mut self) -> &mut GPByteRegisters {
        &mut self.main
    }
}
