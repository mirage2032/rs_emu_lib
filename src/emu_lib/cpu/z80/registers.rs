use std::collections::HashMap;

use crate::emu_lib::cpu::registers::AnyRegister;

use super::super::registers::RegisterOps;
use super::super::registers::UnionRegister;

pub struct AltRegisters {
    pub af: UnionRegister,
    pub bc: UnionRegister,
    pub de: UnionRegister,
    pub hl: UnionRegister,
}

impl AltRegisters {
    pub fn new() -> AltRegisters {
        AltRegisters {
            af: UnionRegister::new(),
            bc: UnionRegister::new(),
            de: UnionRegister::new(),
            hl: UnionRegister::new(),
        }
    }
}


pub struct Registers {
    pub main: AltRegisters,
    pub shadow: AltRegisters,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub i: u8,
    pub r: u8,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            main: AltRegisters::new(),
            shadow: AltRegisters::new(),
            ix: 0,
            iy: 0,
            sp: 0,
            i: 0,
            r: 0,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.main = AltRegisters::new();
        self.shadow = AltRegisters::new();
        self.ix = 0;
        self.iy = 0;
        self.sp = 0;
        self.i = 0;
        self.r = 0;
        self.pc = 0;
    }
}

impl RegisterOps for Registers {
    fn get_all(&self) -> HashMap<&str, AnyRegister> {
        let mut registers = HashMap::new();
        registers.insert("af", AnyRegister::Bit16(self.main.af));
        registers.insert("bc", AnyRegister::Bit16(self.main.bc));
        registers.insert("de", AnyRegister::Bit16(self.main.de));
        registers.insert("hl", AnyRegister::Bit16(self.main.hl));
        registers.insert("af'", AnyRegister::Bit16(self.shadow.af));
        registers.insert("bc'", AnyRegister::Bit16(self.shadow.bc));
        registers.insert("de'", AnyRegister::Bit16(self.shadow.de));
        registers.insert("hl'", AnyRegister::Bit16(self.shadow.hl));
        registers.insert("ix", AnyRegister::Bit16(UnionRegister::new()));
        registers.insert("iy", AnyRegister::Bit16(UnionRegister::new()));
        registers.insert("sp", AnyRegister::Bit16(UnionRegister::new()));
        registers.insert("i", AnyRegister::Bit8(0));
        registers.insert("r", AnyRegister::Bit8(0));
        registers.insert("pc", AnyRegister::Bit16(UnionRegister::new()));
        registers
    }

    fn get8bit(&self, register: &str) -> u8 {
        match register {
            "a" => self.main.af.get_high(),
            "f" => self.main.af.get_low(),
            "b" => self.main.bc.get_high(),
            "c" => self.main.bc.get_low(),
            "d" => self.main.de.get_high(),
            "e" => self.main.de.get_low(),
            "h" => self.main.hl.get_high(),
            "l" => self.main.hl.get_low(),
            "a'" => self.shadow.af.get_high(),
            "f'" => self.shadow.af.get_low(),
            "b'" => self.shadow.bc.get_high(),
            "c'" => self.shadow.bc.get_low(),
            "d'" => self.shadow.de.get_high(),
            "e'" => self.shadow.de.get_low(),
            "h'" => self.shadow.hl.get_high(),
            "l'" => self.shadow.hl.get_low(),
            "i" => self.i,
            "r" => self.r,
            _ => panic!("Invalid register"),
        }
    }

    fn get16bit(&self, register: &str) -> u16 {
        match register {
            "af" => self.main.af.get(),
            "bc" => self.main.bc.get(),
            "de" => self.main.de.get(),
            "hl" => self.main.hl.get(),
            "af'" => self.shadow.af.get(),
            "bc'" => self.shadow.bc.get(),
            "de'" => self.shadow.de.get(),
            "hl'" => self.shadow.hl.get(),
            "ix" => self.ix,
            "iy" => self.iy,
            "sp" => self.sp,
            "pc" => self.pc,
            _ => panic!("Invalid register"),
        }
    }

    fn set8bit(&mut self, register: &str, value: u8) {
        match register {
            "a" => self.main.af.set_high(value),
            "f" => self.main.af.set_low(value),
            "b" => self.main.bc.set_high(value),
            "c" => self.main.bc.set_low(value),
            "d" => self.main.de.set_high(value),
            "e" => self.main.de.set_low(value),
            "h" => self.main.hl.set_high(value),
            "l" => self.main.hl.set_low(value),
            "a'" => self.shadow.af.set_high(value),
            "f'" => self.shadow.af.set_low(value),
            "b'" => self.shadow.bc.set_high(value),
            "c'" => self.shadow.bc.set_low(value),
            "d'" => self.shadow.de.set_high(value),
            "e'" => self.shadow.de.set_low(value),
            "h'" => self.shadow.hl.set_high(value),
            "l'" => self.shadow.hl.set_low(value),
            _ => panic!("Invalid register"),
        }
    }
    fn set16bit(&mut self, register: &str, value: u16) {
        match register {
            "af" => self.main.af.set(value),
            "bc" => self.main.bc.set(value),
            "de" => self.main.de.set(value),
            "hl" => self.main.hl.set(value),
            "af'" => self.shadow.af.set(value),
            "bc'" => self.shadow.bc.set(value),
            "de'" => self.shadow.de.set(value),
            "hl'" => self.shadow.hl.set(value),
            "ix" => self.ix = value,
            "iy" => self.iy = value,
            "sp" => self.sp = value,
            "pc" => self.pc = value,
            _ => panic!("Invalid register"),
        }
    }
    fn increment_pc(&mut self, value: u16) {
        self.pc += value;
    }
}