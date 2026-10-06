use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::alu;
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct SBC_A_PIDXD<R: IndexRegister> {
    common: InstructionCommon,
    d: i8,
    index: PhantomData<R>,
}

pub type SBC_A_PIXD = SBC_A_PIDXD<IX>;
pub type SBC_A_PIYD = SBC_A_PIDXD<IY>;

impl<R: IndexRegister> SBC_A_PIDXD<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        Ok(Self {
            common: InstructionCommon::new(3, 19, true),
            d: memory.read_8(pos.wrapping_add(2))? as i8,
            index: PhantomData,
        })
    }

    pub fn new_with_value(d: u8) -> Self {
        Self {
            common: InstructionCommon::new(3, 19, true),
            d: d as i8,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for SBC_A_PIDXD<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SBC A, ({name}+0x{:02X})", self.d as u8, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for SBC_A_PIDXD<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x9e, self.d as u8]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for SBC_A_PIDXD<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let offset = R::get(&cpu.registers).wrapping_add(self.d as u16);
        let value = memory.read_8(offset as u16)?;
        let carry = cpu.registers.gp.f.carry();
        cpu.registers.gp.a = alu::sub8(&mut cpu.registers.gp.f, cpu.registers.gp.a, value, carry);
        Ok(())
    }
}
