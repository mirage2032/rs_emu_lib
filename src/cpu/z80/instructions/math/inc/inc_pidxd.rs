use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::alu;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::errors::MemoryReadError;
use crate::memory::Memory;
use crate::memory::MemoryDevice;

#[derive(Debug)]
pub struct INC_PIDXD<R: IndexRegister> {
    common: InstructionCommon,
    d: i8,
    index: PhantomData<R>,
}

pub type INC_PIXD = INC_PIDXD<IX>;
pub type INC_PIYD = INC_PIDXD<IY>;

impl<R: IndexRegister> INC_PIDXD<R> {
    pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
        Ok(Self::new_with_value(memory.read_8(pos.wrapping_add(2))? as i8))
    }

    pub fn new_with_value(d: i8) -> Self {
        Self {
            common: InstructionCommon::new(3, 23, true),
            d,
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for INC_PIDXD<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC ({name}+0x{:02X})", self.d, name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for INC_PIDXD<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0x34, self.d as u8]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for INC_PIDXD<R> {
    fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        let address = R::get(&cpu.registers).wrapping_add(self.d as u16);
        let value = memory.read_8(address)?;
        let result = alu::inc8(&mut cpu.registers.gp.f, value);
        memory.write_8(address, result)?;
        Ok(())
    }
}
