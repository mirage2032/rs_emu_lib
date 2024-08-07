use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use bitfield_struct::bitfield;

use crate::cpu::instruction::BaseInstruction;
use crate::memory::Memory;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GPRegister {
    Bit8(u8),
    Bit16(u16),
}

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
pub struct Flags {
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
#[derive(Default, Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
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

pub trait RegisterOps: Debug {
    fn clear(&mut self);
    fn set_8(&mut self, register: &str, value: u8);
    fn set_16(&mut self, register: &str, value: u16);
    fn get_8(&self, register: &str) -> u8;
    fn get_16(&self, register: &str) -> u16;
    fn get_all(&self) -> HashMap<&str, GPRegister>;
    fn pc(&self) -> &u16;
    fn pc_mut(&mut self) -> &mut u16;
    fn sp(&self) -> &u16;
    fn sp_mut(&mut self) -> &mut u16;
    fn get_gp(&self) -> &GPByteRegisters;
    fn get_gp_mut(&mut self) -> &mut GPByteRegisters;
}

pub trait InstructionParser {
    fn ins_from_mem(&self, memory: &Memory, pos: u16)
        -> Result<Box<(dyn BaseInstruction)>, String>;
    fn ins_from_vec(&self, memory: Vec<u8>, pos: u16)
        -> Result<Box<(dyn BaseInstruction)>, String>;
    fn ins_from_string(&self, instruction: &String) -> Result<Box<(dyn BaseInstruction)>, String>;
}
