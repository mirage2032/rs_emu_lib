use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

use bitfield_struct::bitfield;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseRegister {
    Bit8(u8),
    Bit16(u16),
}

impl Display for BaseRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseRegister::Bit8(val) => write!(f, "{:02X}", val),
            BaseRegister::Bit16(val) => write!(f, "{:04X}", val),
        }
    }
}

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct Flags {
    pub carry: bool,
    pub add_sub: bool,
    pub parity_overflow: bool,
    pub bit3: bool,
    pub half_carry: bool,
    pub bit5: bool,
    pub zero: bool,
    pub sign: bool,
}

#[cfg(target_endian = "big")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GPByteRegisters {
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
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GPByteRegisters {
    pub f: Flags,
    pub a: u8,
    pub c: u8,
    pub b: u8,
    pub e: u8,
    pub d: u8,
    pub l: u8,
    pub h: u8,
}

impl Default for GPByteRegisters {
    fn default() -> Self {
        GPByteRegisters {
            f: Flags::default(),
            a: 0,
            c: 0,
            b: 0,
            e: 0,
            d: 0,
            l: 0,
            h: 0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct GPWordRegisters {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
}

impl Deref for GPByteRegisters {
    type Target = GPWordRegisters;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl DerefMut for GPByteRegisters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, Clone)]
pub struct AllRegisters {
    pub gp: Vec<GPByteRegisters>,
    pub sp: u16,
    pub pc: u16,
    pub other: HashMap<&'static str, BaseRegister>,
}
