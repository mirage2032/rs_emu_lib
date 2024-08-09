use std::collections::HashMap;
use crate::cpu::registers::{AllRegisters, AllRegistersMut, GPRegisterMut};
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

    fn set_8(&mut self, register: &str, value: u8) -> Result<(), &str>{
        let val =match register {
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
            _ => return Err("Invalid register"),
        };
        Ok(val)
    }

    fn set_16(&mut self, register: &str, value: u16) -> Result<(), &str>{
        let val = match register {
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
            _ => return Err("Invalid register"),
        };
        Ok(val)
    }

    fn get_8(&self, register: &str) -> Result<u8,&str> {
        let val = match register {
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
            _ => return Err("Invalid register"),
        };
        Ok(val)
    }
    fn get_16(&self, register: &str) -> Result<u16, &str>{
        let val = match register {
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
            _ => return Err("Invalid register"),
        };
        Ok(val)
    }

    fn get_all(&self) -> AllRegisters {
        let mut map = HashMap::new();
        map.insert("ix", GPRegister::Bit16(&self.ix));
        map.insert("iy", GPRegister::Bit16(&self.iy));
        map.insert("i", GPRegister::Bit8(&self.i));
        map.insert("r", GPRegister::Bit8(&self.r));
        AllRegisters {
            gp: vec![&self.main, &self.shadow],
            sp: &self.sp,
            pc: &self.pc,
            other: map,
        }
    }

    fn get_all_mut(&mut self) -> AllRegistersMut {
        let mut map = HashMap::new();
        map.insert("ix", GPRegisterMut::Bit16(&mut self.ix));
        map.insert("iy", GPRegisterMut::Bit16(&mut self.iy));
        map.insert("i", GPRegisterMut::Bit8(&mut self.i));
        map.insert("r", GPRegisterMut::Bit8(&mut self.r));
        AllRegistersMut {
            gp: vec![&mut self.main, &mut self.shadow],
            sp: &mut self.sp,
            pc: &mut self.pc,
            other: map,
        }
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
